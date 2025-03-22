use crate::*;
use std::{cell::UnsafeCell, mem::size_of, ops::Deref};

/// Wrapper around an [`UnsafeCell`] (either mutable reference or owned)
/// that divides the underlying slice in chunks.
#[derive(Debug)]
pub(crate) struct UnsafeCellChunkSlice<B> {
    inner: B,
    len: usize,
    chunk_size: usize,
}

// Safety: access paradigms shift responsability to the user to ensure
// no data races happen.
unsafe impl<T: Send + Sync> Sync for UnsafeCellChunkSlice<&mut UnsafeCell<[T]>> {}
unsafe impl<T: Send + Sync> Sync for UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>> {}

impl<T> From<UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>>> for Box<[T]> {
    #[inline]
    fn from(value: UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>>) -> Self {
        value.into_inner()
    }
}

impl<T> From<UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>>> for Vec<T> {
    #[inline]
    fn from(value: UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>>) -> Self {
        value.into_inner().into_vec()
    }
}

impl<'a, T> UnsafeCellChunkSlice<&'a mut UnsafeCell<[T]>> {
    /// Creates a new borrowed slice with chunks of `chunk_size`.
    ///
    /// # Panics
    ///
    /// Panics if `slice.len()` is not divisible by `chunk_size`.
    pub(crate) fn new_borrowed(slice: &'a mut [T], chunk_size: usize) -> Self {
        assert_eq!(slice.len() % chunk_size, 0);
        let len = slice.len() / chunk_size;

        Self {
            inner: UnsafeCell::from_mut(slice),
            len,
            chunk_size,
        }
    }
}

impl<T> UnsafeCellChunkSlice<Box<UnsafeCell<[T]>>> {
    /// Creates a new owned slice with chunks of `chunk_size`.
    ///
    /// # Panics
    ///
    /// Panics if `slice.len()` is not divisible by `chunk_size`.
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

    /// Extracts the inner boxed slice from the wrapper.
    fn into_inner(self) -> Box<[T]> {
        let ptr = Box::into_raw(self.inner) as *mut [T];
        unsafe {
            // Safety: pointer is owned and repr is transparent
            Box::from_raw(ptr)
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> TrustedSizedCollection
    for UnsafeCellChunkSlice<B>
{
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> TrustedChunkSizedCollection
    for UnsafeCellChunkSlice<B>
{
    #[inline]
    fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    #[inline]
    fn num_elements(&self) -> usize {
        self.inner.get().len()
    }

    #[inline]
    fn num_chunks(&self) -> usize {
        self.len
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerIndex<[T]> for UnsafeCellChunkSlice<B> {
    #[inline]
    unsafe fn get_ptr_unchecked(&self, index: usize) -> *const [T] {
        self.get_mut_ptr_unchecked(index) as *const [T]
    }

    #[inline]
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

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerChunkIndex<T>
    for UnsafeCellChunkSlice<B>
{
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeNoRefChunkIndex<T>
    for UnsafeCellChunkSlice<B>
{
    #[inline]
    unsafe fn get_values_unchecked<O: AsMut<[T]>>(&self, index: usize, mut out: O) -> O
    where
        T: Copy,
    {
        let slice = out.as_mut();
        debug_assert!(index < self.len);
        debug_assert_eq!(slice.len(), self.chunk_size);

        let fat_ptr = self.get_ptr_unchecked(index);
        debug_assert_eq!(fat_ptr.len(), self.chunk_size);
        let mut ptr = fat_ptr as *const T;

        for elem in slice {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                *elem = *ptr;

                // Safety: object is allocated and the caller guarantees that
                // ptr is in bounds
                ptr = ptr.add(1);
            }
        }

        out
    }

    #[inline]
    unsafe fn set_values_unchecked(&self, index: usize, value: &[T])
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

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeIndex<[T]> for UnsafeCellChunkSlice<B> {
    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> &[T] {
        unsafe {
            // Safety: the caller guarantees Rust's aliasing rules are respected and that
            // index is valid
            &*self.get_ptr_unchecked(index)
        }
    }

    #[inline]
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut [T] {
        unsafe {
            // Safety: the caller guarantees Rust's aliasing rules are respected and that
            // index is valid
            &mut *self.get_mut_ptr_unchecked(index)
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeChunkIndex<T> for UnsafeCellChunkSlice<B> {}
