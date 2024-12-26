use crate::*;

pub unsafe trait PointerAccess<T: ?Sized>: TrustedSizedCollection {
    unsafe fn get_ptr_unchecked(&self, index: usize) -> *const T;

    unsafe fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T;

    #[inline(always)]
    fn get_ptr(&self, index: usize) -> *const T {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: the caller is responsible for ensuring index is in bounds
            self.get_ptr_unchecked(index)
        }
    }

    #[inline(always)]
    fn get_mut_ptr(&self, index: usize) -> *mut T {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: the caller is responsible for ensuring index is in bounds
            self.get_mut_ptr_unchecked(index)
        }
    }
}

pub unsafe trait PointerChunkAccess<T>:
    PointerAccess<[T]> + TrustedChunkSizedCollection
{
}
