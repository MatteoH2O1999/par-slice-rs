use super::*;

pub trait UnsafeParSlice<T: ?Sized>: PointerParSlice<T> {
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

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    unsafe fn get_mut(&self, index: usize) -> &mut T {
        unsafe {
            // Safety: the caller must guarantee that no other references with the same index
            // exists for the duration of the returned reference
            &mut *self.get_mut_ptr(index)
        }
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut T {
        unsafe {
            // Safety: the caller must guarantee that no other references with the same index
            // exists for the duration of the returned reference
            &mut *self.get_mut_ptr_unchecked(index)
        }
    }
}
