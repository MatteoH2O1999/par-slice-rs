use crate::*;
use std::{cell::UnsafeCell, mem::size_of, ops::Deref};

pub(crate) struct UnsafeCellSlice<B>(B);

unsafe impl<T: Sync> Sync for UnsafeCellSlice<&UnsafeCell<[T]>> {}
unsafe impl<T: Sync> Sync for UnsafeCellSlice<Box<UnsafeCell<[T]>>> {}

impl<T> From<UnsafeCellSlice<Box<UnsafeCell<[T]>>>> for Box<[T]> {
    fn from(value: UnsafeCellSlice<Box<UnsafeCell<[T]>>>) -> Self {
        let ptr = Box::into_raw(value.0) as *mut [T];
        unsafe {
            // Safety: pointer is owned and repr is transparent
            Box::from_raw(ptr)
        }
    }
}

impl<T> UnsafeCellSlice<&UnsafeCell<[T]>> {
    pub(crate) fn new_borrowed(slice: &mut [T]) -> Self {
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
    pub(crate) fn new_owned(slice: Box<[T]>) -> Self {
        let ptr = Box::into_raw(slice) as *mut UnsafeCell<[T]>;
        let boxed = unsafe {
            // Safety: UnsafeCell is repr(transparent)
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

        let ptr = self.0.get() as *mut T;
        unsafe {
            // Safety: ptr is derived from an allocated object so cannot be bigger
            // than isize::MAX bytes
            ptr.add(index)
        }
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

unsafe impl<T: Sync> Sync for UnsafeCellChunkSlice<&UnsafeCell<[T]>> {}
unsafe impl<T: Sync> Sync for UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>> {}

impl<T> From<UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>>> for Box<[T]> {
    fn from(value: UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>>) -> Self {
        let ptr = Box::into_raw(value.inner) as *mut [T];
        unsafe {
            // Safety: pointer is owned and repr is transparent
            Box::from_raw(ptr)
        }
    }
}

impl<T> UnsafeCellChunkSlice<&UnsafeCell<[T]>> {
    pub(crate) fn new_borrowed(slice: &mut [T], chunk_size: usize) -> Self {
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
    pub(crate) fn new_owned(slice: Box<[T]>, chunk_size: usize) -> Self {
        assert_eq!(slice.len() % chunk_size, 0);
        let len = slice.len() / chunk_size;

        let ptr = Box::into_raw(slice) as *mut UnsafeCell<[T]>;
        let boxed = unsafe {
            // Safety: UnsafeCell is repr(transparent)
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
