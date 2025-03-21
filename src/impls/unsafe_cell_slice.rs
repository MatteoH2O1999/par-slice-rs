use crate::*;
use std::{cell::UnsafeCell, mem::size_of, ops::Deref};

/// Wrapper around an [`UnsafeCell`] (either mutable reference or owned).
#[derive(Debug)]
pub(crate) struct UnsafeCellSlice<B>(B);

// Safety: access paradigms shift responsability to the user to ensure
// no data races happen.
unsafe impl<T: Send + Sync> Sync for UnsafeCellSlice<&mut UnsafeCell<[T]>> {}
unsafe impl<T: Send + Sync> Sync for UnsafeCellSlice<Box<UnsafeCell<[T]>>> {}

impl<T> From<UnsafeCellSlice<Box<UnsafeCell<[T]>>>> for Box<[T]> {
    #[inline]
    fn from(value: UnsafeCellSlice<Box<UnsafeCell<[T]>>>) -> Self {
        value.into_inner()
    }
}

impl<T> From<UnsafeCellSlice<Box<UnsafeCell<[T]>>>> for Vec<T> {
    #[inline]
    fn from(value: UnsafeCellSlice<Box<UnsafeCell<[T]>>>) -> Self {
        value.into_inner().into_vec()
    }
}

impl<'a, T> UnsafeCellSlice<&'a mut UnsafeCell<[T]>> {
    /// Creates a new borrowed slice.
    pub(crate) fn new_borrowed(slice: &'a mut [T]) -> Self {
        Self(UnsafeCell::from_mut(slice))
    }
}

impl<T> UnsafeCellSlice<Box<UnsafeCell<[T]>>> {
    /// Creates a new owned slice.
    pub(crate) fn new_owned(slice: Box<[T]>) -> Self {
        let ptr = Box::into_raw(slice) as *mut UnsafeCell<[T]>;
        let boxed = unsafe {
            // Safety: UnsafeCell is repr(transparent)
            Box::from_raw(ptr)
        };
        Self(boxed)
    }

    /// Extracts the inner boxed slice from the wrapper.
    fn into_inner(self) -> Box<[T]> {
        let ptr = Box::into_raw(self.0) as *mut [T];
        unsafe {
            // Safety: pointer is owned and repr is transparent
            Box::from_raw(ptr)
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> TrustedSizedCollection<T>
    for UnsafeCellSlice<B>
{
    #[inline]
    fn len(&self) -> usize {
        self.0.get().len()
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerIndex<T> for UnsafeCellSlice<B> {
    #[inline]
    unsafe fn get_ptr_unchecked(&self, index: usize) -> *const T {
        self.get_mut_ptr_unchecked(index) as *const T
    }

    #[inline]
    unsafe fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T {
        debug_assert!(index < self.len());
        debug_assert!(index * size_of::<T>() < isize::MAX as usize);

        let ptr = self.0.get() as *mut T;
        unsafe {
            // Safety: the caller guarantees index is valid
            ptr.add(index)
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeNoRefIndex<T> for UnsafeCellSlice<B> {
    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> T
    where
        T: Copy,
    {
        unsafe {
            // Safety: the caller guarantees that there are no data races and that
            // index is valid
            *self.get_ptr_unchecked(index)
        }
    }

    #[inline]
    unsafe fn set_unchecked(&self, index: usize, value: T)
    where
        T: Sized,
    {
        unsafe {
            // Safety: the caller guarantees that there are no data races and that
            // index is valid
            *self.get_mut_ptr_unchecked(index) = value
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeIndex<T> for UnsafeCellSlice<B> {
    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe {
            // Safety: the caller guarantees Rust's aliasing rules are respected and that
            // index is valid
            &*self.get_ptr_unchecked(index)
        }
    }

    #[inline]
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut T {
        unsafe {
            // Safety: the caller guarantees Rust's aliasing rules are respected and that
            // index is valid
            &mut *self.get_mut_ptr_unchecked(index)
        }
    }
}
