use crate::{IntoParSlice, ParSliceView, PointerParSlice};
use std::{
    cell::UnsafeCell,
    mem::size_of,
    ops::{Deref, DerefMut},
};

pub(crate) struct UnsafeCellSlice<B>(B);

impl<T> UnsafeCellSlice<&UnsafeCell<[T]>> {
    fn new_borrowed(slice: &mut [T]) -> Self {
        // TODO: replace with UnsafeCell::from_mut when stable
        let ptr = slice as *mut [T] as *mut UnsafeCell<[T]>;
        let unsafe_slice = unsafe {
            // Safety: UnsafeCell is repr(transparent)
            &mut *ptr
        };
        Self(unsafe_slice)
    }
}

impl<T> UnsafeCellSlice<Box<UnsafeCell<[T]>>> {
    unsafe fn new_owned(ptr: *mut [T]) -> Self {
        let ptr = ptr as *mut UnsafeCell<[T]>;
        let boxed = unsafe {
            // Safety: UnsafeCell is repr(transparent) and caller guarantees
            // ownership of the allocation pointed to by ptr
            Box::from_raw(ptr)
        };
        Self(boxed)
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerParSlice<T> for UnsafeCellSlice<B> {
    #[inline(always)]
    fn get_ptr_unchecked(&self, index: usize) -> *const T {
        self.get_mut_ptr_unchecked(index) as *const T
    }

    #[inline(always)]
    fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T {
        debug_assert!(index < self.len());
        debug_assert!(index * size_of::<T>() < isize::MAX as usize);

        let mut ptr = self.0.get() as *mut T;
        unsafe {
            // Safety: ptr is derived from an allocated object so cannot be bigger
            // than isize::MAX bytes
            ptr = ptr.add(index);
        }
        ptr
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.get().len()
    }
}

pub(crate) struct UnsafeCellChunkSlice<B> {
    inner: B,
    len: usize,
    chunk_size: usize,
}

impl<T> UnsafeCellChunkSlice<&UnsafeCell<[T]>> {
    fn new_borrowed(slice: &mut [T], chunk_size: usize) -> Self {
        assert_eq!(slice.len() % chunk_size, 0);
        let len = slice.len() / chunk_size;

        // TODO: replace with UnsafeCell::from_mut when stable
        let ptr = slice as *mut [T] as *mut UnsafeCell<[T]>;
        let unsafe_slice = unsafe {
            // Safety: UnsafeCell is repr(transparent)
            &mut *ptr
        };
        Self {
            inner: unsafe_slice,
            len,
            chunk_size,
        }
    }
}

impl<T> UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>> {
    unsafe fn new_owned(ptr: *mut [T], chunk_size: usize) -> Self {
        assert_eq!(ptr.len() % chunk_size, 0);
        let len = ptr.len() / chunk_size;

        let ptr = ptr as *mut UnsafeCell<[T]>;
        let boxed = unsafe {
            // Safety: UnsafeCell is repr(transparent) and caller guarantees
            // ownership of the allocation pointed to by ptr
            Box::from_raw(ptr)
        };

        Self {
            inner: boxed,
            len,
            chunk_size,
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerParSlice<[T]>
    for UnsafeCellChunkSlice<B>
{
    #[inline(always)]
    fn get_ptr_unchecked(&self, index: usize) -> *const [T] {
        self.get_mut_ptr_unchecked(index) as *const [T]
    }

    #[inline(always)]
    fn get_mut_ptr_unchecked(&self, index: usize) -> *mut [T] {
        debug_assert!(index < self.len());

        let offset = index * self.chunk_size;
        debug_assert!(offset * size_of::<T>() < isize::MAX as usize);

        let mut ptr = self.inner.get() as *mut T;
        unsafe {
            // Safety: ptr is derived from an allocated object so cannot be bigger
            // than isize::MAX bytes
            ptr = ptr.add(offset);
        }
        std::ptr::slice_from_raw_parts_mut(ptr, self.chunk_size)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> ParSliceView<T> for [T] {
    #[inline(always)]
    fn as_pointer_par_slice(&mut self) -> impl PointerParSlice<T> {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_data_race_par_slice(&mut self) -> impl crate::UnsafeDataRaceParSlice<T> {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_unsafe_par_slice(&mut self) -> impl crate::UnsafeParSlice<T> {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_pointer_par_chunk_slice(&mut self, chunk_size: usize) -> impl PointerParSlice<[T]> {
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl crate::UnsafeDataRaceParChunkSlice<T> {
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_unsafe_par_chunk_slice(&mut self, chunk_size: usize) -> impl crate::UnsafeParSlice<[T]> {
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }
}

impl<T, B: Deref<Target = [T]> + DerefMut> IntoParSlice<T> for B {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerParSlice<T> {
        let mut manually_drop = std::mem::ManuallyDrop::new(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellSlice::new_owned(manually_drop.deref_mut().deref_mut())
        }
    }

    #[inline(always)]
    fn into_data_race_par_slice(self) -> impl crate::UnsafeDataRaceParSlice<T> {
        let mut manually_drop = std::mem::ManuallyDrop::new(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellSlice::new_owned(manually_drop.deref_mut().deref_mut())
        }
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl crate::UnsafeParSlice<T> {
        let mut manually_drop = std::mem::ManuallyDrop::new(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellSlice::new_owned(manually_drop.deref_mut().deref_mut())
        }
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(self, chunk_size: usize) -> impl PointerParSlice<[T]> {
        let mut manually_drop = std::mem::ManuallyDrop::new(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellChunkSlice::new_owned(manually_drop.deref_mut().deref_mut(), chunk_size)
        }
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl crate::UnsafeDataRaceParChunkSlice<T> {
        let mut manually_drop = std::mem::ManuallyDrop::new(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellChunkSlice::new_owned(manually_drop.deref_mut().deref_mut(), chunk_size)
        }
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(self, chunk_size: usize) -> impl crate::UnsafeParSlice<[T]> {
        let mut manually_drop = std::mem::ManuallyDrop::new(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellChunkSlice::new_owned(manually_drop.deref_mut().deref_mut(), chunk_size)
        }
    }
}
