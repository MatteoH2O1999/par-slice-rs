use crate::*;
use std::{cell::UnsafeCell, mem::size_of, ops::Deref};

#[derive(Debug)]
pub(crate) struct UnsafeCellChunkSlice<B> {
    inner: B,
    len: usize,
    chunk_size: usize,
}

unsafe impl<T: Sync> Sync for UnsafeCellChunkSlice<&mut UnsafeCell<[T]>> {}
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

impl<T> UnsafeCellChunkSlice<&mut UnsafeCell<[T]>> {
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

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> TrustedSizedCollection
    for UnsafeCellChunkSlice<B>
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> TrustedChunkSizedCollection
    for UnsafeCellChunkSlice<B>
{
    #[inline(always)]
    fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    #[inline(always)]
    fn num_elements(&self) -> usize {
        self.inner.get().len()
    }

    #[inline(always)]
    fn num_chunks(&self) -> usize {
        self.len
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerAccess<[T]> for UnsafeCellChunkSlice<B> {
    #[inline(always)]
    unsafe fn get_ptr_unchecked(&self, index: usize) -> *const [T] {
        self.get_mut_ptr_unchecked(index) as *const [T]
    }

    #[inline(always)]
    unsafe fn get_mut_ptr_unchecked(&self, index: usize) -> *mut [T] {
        debug_assert!(index < self.len());

        let offset = index * self.chunk_size;
        debug_assert!(offset * size_of::<T>() < isize::MAX as usize);

        let mut ptr = self.inner.get() as *mut T;
        unsafe {
            // Safety: caller is responsible for guaranteeing that
            // offset stays in bounds of allocated object
            ptr = ptr.add(offset);
        }
        std::ptr::slice_from_raw_parts_mut(ptr, self.chunk_size)
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerChunkAccess<T>
    for UnsafeCellChunkSlice<B>
{
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeDataRaceChunkAccess<T>
    for UnsafeCellChunkSlice<B>
{
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> Box<[T]>
    where
        T: Copy,
    {
        debug_assert!(index < self.len);
        let fat_ptr = self.get_ptr_unchecked(index);

        let mut res = Box::new_uninit_slice(fat_ptr.len());
        let mut ptr = fat_ptr as *const T;

        for elem in res.iter_mut() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                elem.write(*ptr);

                // Safety: object is allocated and the caller guarantees that
                // ptr is in bounds
                ptr = ptr.add(1);
            }
        }

        unsafe {
            // Safety: the slice is filled with the correct elements
            res.assume_init()
        }
    }

    #[inline(always)]
    unsafe fn set_unchecked(&self, index: usize, value: &[T])
    where
        T: Clone,
    {
        debug_assert!(index < self.len);
        debug_assert_eq!(value.len(), self.chunk_size);

        let mut ptr = self.get_mut_ptr_unchecked(index) as *mut T;

        for elem in value.iter() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                *ptr = elem.clone();

                // Safety: object is allocated and the caller guarantees that
                // ptr is in bounds
                ptr = ptr.add(1);
            }
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeAccess<[T]> for UnsafeCellChunkSlice<B> {
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

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeChunkAccess<T>
    for UnsafeCellChunkSlice<B>
{
}
