use crate::PointerParSlice;
use std::{cell::UnsafeCell, mem::size_of};

impl<T> PointerParSlice<T> for UnsafeCell<[T]> {
    #[inline(always)]
    fn get_ptr_unchecked(&self, index: usize) -> *const T {
        self.get_mut_ptr_unchecked(index) as *const T
    }

    #[inline(always)]
    fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T {
        debug_assert!(index < self.len());
        debug_assert!(index * size_of::<T>() < isize::MAX as usize);

        let mut ptr = self.get() as *mut T;
        unsafe {
            // Safety: ptr is derived from an allocated object so cannot be bigger
            // than isize::MAX bytes
            ptr = ptr.add(index);
        }
        ptr
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.get().len()
    }
}
