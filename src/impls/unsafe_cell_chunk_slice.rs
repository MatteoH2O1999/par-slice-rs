use crate::*;
use std::{cell::UnsafeCell, mem::size_of, ops::Deref};

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

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerAccess<[T]> for UnsafeCellChunkSlice<B> {
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

impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeDataRaceChunkAccess<T>
    for UnsafeCellChunkSlice<B>
{
    #[inline(always)]
    unsafe fn get(&self, index: usize) -> Box<[T]>
    where
        T: Copy,
    {
        let fat_ptr = self.get_ptr(index);

        let mut res = Box::new_uninit_slice(fat_ptr.len());
        let mut ptr = fat_ptr as *const T;

        for elem in res.iter_mut() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                elem.write(*ptr);

                // Safety: size_of::<T>() is < isize::MAX as it is allocated
                ptr = ptr.add(1);
            }
        }

        unsafe {
            // Safety: the slice is filled with the correct elements
            res.assume_init()
        }
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> Box<[T]>
    where
        T: Copy,
    {
        let fat_ptr = self.get_ptr_unchecked(index);

        let mut res = Box::new_uninit_slice(fat_ptr.len());
        let mut ptr = fat_ptr as *const T;

        for elem in res.iter_mut() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                elem.write(*ptr);

                // Safety: size_of::<T>() is < isize::MAX as it is allocated
                ptr = ptr.add(1);
            }
        }

        unsafe {
            // Safety: the slice is filled with the correct elements
            res.assume_init()
        }
    }

    #[inline(always)]
    unsafe fn set(&self, index: usize, value: &[T])
    where
        T: Clone,
    {
        let fat_ptr = self.get_mut_ptr(index);
        assert_eq!(value.len(), fat_ptr.len());

        let mut ptr = fat_ptr as *mut T;

        for elem in value.iter() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                *ptr = elem.clone();

                // Safety: size_of::<T>() is < isize::MAX as it is allocated
                ptr = ptr.add(1);
            }
        }
    }

    #[inline(always)]
    unsafe fn set_unchecked(&self, index: usize, value: &[T])
    where
        T: Clone,
    {
        let fat_ptr = self.get_mut_ptr_unchecked(index);
        debug_assert_eq!(value.len(), fat_ptr.len());

        let mut ptr = fat_ptr as *mut T;

        for elem in value.iter() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                *ptr = elem.clone();

                // Safety: size_of::<T>() is < isize::MAX as it is allocated
                ptr = ptr.add(1);
            }
        }
    }
}

impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeAccess<[T]> for UnsafeCellChunkSlice<B> {
    #[inline(always)]
    unsafe fn get(&self, index: usize) -> &[T] {
        unsafe {
            // Safety: the caller must guarantee not to modify the memory pointed
            // by this reference for the duration of its lifetime and not to create
            // a &mut reference with `get_mut`
            &*self.get_ptr(index)
        }
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> &[T] {
        unsafe {
            // Safety: the caller must guarantee not to modify the memory pointed
            // by this reference for the duration of its lifetime and not to create
            // a &mut reference with `get_mut`
            &*self.get_ptr_unchecked(index)
        }
    }

    #[inline(always)]
    unsafe fn get_mut(&self, index: usize) -> &mut [T] {
        unsafe {
            // Safety: the caller must guarantee that no other references with the same index
            // exists for the duration of the returned reference
            &mut *self.get_mut_ptr(index)
        }
    }

    #[inline(always)]
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut [T] {
        unsafe {
            // Safety: the caller must guarantee that no other references with the same index
            // exists for the duration of the returned reference
            &mut *self.get_mut_ptr_unchecked(index)
        }
    }
}
