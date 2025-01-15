use crate::*;
use std::{cell::UnsafeCell, mem::size_of, ops::Deref};

#[derive(Debug)]
pub(crate) struct UnsafeCellSlice<B>(B);

unsafe impl<T: Sync> Sync for UnsafeCellSlice<&mut UnsafeCell<[T]>> {}
unsafe impl<T: Sync> Sync for UnsafeCellSlice<Box<UnsafeCell<[T]>>> {}

impl<T> From<UnsafeCellSlice<Box<UnsafeCell<[T]>>>> for Box<[T]> {
    #[inline(always)]
    fn from(value: UnsafeCellSlice<Box<UnsafeCell<[T]>>>) -> Self {
        value.into_inner()
    }
}

impl<T> From<UnsafeCellSlice<Box<UnsafeCell<[T]>>>> for Vec<T> {
    #[inline(always)]
    fn from(value: UnsafeCellSlice<Box<UnsafeCell<[T]>>>) -> Self {
        value.into_inner().into_vec()
    }
}

impl<'a, T> UnsafeCellSlice<&'a mut UnsafeCell<[T]>> {
    #[inline(always)]
    pub(crate) fn new_borrowed(slice: &'a mut [T]) -> Self {
        Self(UnsafeCell::from_mut(slice))
    }
}

impl<T> UnsafeCellSlice<Box<UnsafeCell<[T]>>> {
    #[inline(always)]
    pub(crate) fn new_owned(slice: Box<[T]>) -> Self {
        let ptr = Box::into_raw(slice) as *mut UnsafeCell<[T]>;
        let boxed = unsafe {
            // Safety: UnsafeCell is repr(transparent)
            Box::from_raw(ptr)
        };
        Self(boxed)
    }

    #[inline(always)]
    fn into_inner(self) -> Box<[T]> {
        let ptr = Box::into_raw(self.0) as *mut [T];
        unsafe {
            // Safety: pointer is owned and repr is transparent
            Box::from_raw(ptr)
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> TrustedSizedCollection for UnsafeCellSlice<B> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.get().len()
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> PointerAccess<T> for UnsafeCellSlice<B> {
    #[inline(always)]
    unsafe fn get_ptr_unchecked(&self, index: usize) -> *const T {
        self.get_mut_ptr_unchecked(index) as *const T
    }

    #[inline(always)]
    unsafe fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T {
        debug_assert!(index < self.len());
        debug_assert!(index * size_of::<T>() < isize::MAX as usize);

        let ptr = self.0.get() as *mut T;
        unsafe {
            // Safety: caller is responsible for guaranteeing that
            // offset stays in bounds of allocated object
            ptr.add(index)
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeDataRaceAccess<T> for UnsafeCellSlice<B> {
    #[inline(always)]
    unsafe fn get(&self, index: usize) -> T
    where
        T: Copy,
    {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            *self.get_ptr(index)
        }
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> T
    where
        T: Copy,
    {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            *self.get_ptr_unchecked(index)
        }
    }

    #[inline(always)]
    unsafe fn set(&self, index: usize, value: T)
    where
        T: Sized,
    {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            *self.get_mut_ptr(index) = value
        }
    }

    #[inline(always)]
    unsafe fn set_unchecked(&self, index: usize, value: T)
    where
        T: Sized,
    {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            *self.get_mut_ptr_unchecked(index) = value
        }
    }
}

unsafe impl<T, B: Deref<Target = UnsafeCell<[T]>>> UnsafeAccess<T> for UnsafeCellSlice<B> {
    #[inline(always)]
    unsafe fn get(&self, index: usize) -> &T {
        unsafe {
            // Safety: the caller must guarantee not to modify the memory pointed
            // by this reference for the duration of its lifetime and not to create
            // a &mut reference with `get_mut`
            &*self.get_ptr(index)
        }
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe {
            // Safety: the caller must guarantee not to modify the memory pointed
            // by this reference for the duration of its lifetime and not to create
            // a &mut reference with `get_mut`
            &*self.get_ptr_unchecked(index)
        }
    }

    #[inline(always)]
    unsafe fn get_mut(&self, index: usize) -> &mut T {
        unsafe {
            // Safety: the caller must guarantee that no other references with the same index
            // exists for the duration of the returned reference
            &mut *self.get_mut_ptr(index)
        }
    }

    #[inline(always)]
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut T {
        unsafe {
            // Safety: the caller must guarantee that no other references with the same index
            // exists for the duration of the returned reference
            &mut *self.get_mut_ptr_unchecked(index)
        }
    }
}
