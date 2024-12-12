pub trait PointerParSlice<T> {
    fn len(&self) -> usize;

    fn get_ptr_unchecked(&self, index: usize) -> *const T;

    fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    fn get_ptr(&self, index: usize) -> *const T {
        assert!(index < self.len());
        self.get_ptr_unchecked(index)
    }

    #[inline(always)]
    fn get_mut_ptr(&self, index: usize) -> *mut T {
        assert!(index < self.len());
        self.get_mut_ptr_unchecked(index)
    }
}

pub trait UnsafeDataRaceParSlice<T>: PointerParSlice<T> {
    #[inline(always)]
    unsafe fn set(&self, index: usize, value: T) {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            *self.get_mut_ptr(index) = value
        }
    }

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
    unsafe fn set_unchecked(&self, index: usize, value: T) {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            *self.get_mut_ptr_unchecked(index) = value
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
}

impl<T, I: PointerParSlice<T>> UnsafeDataRaceParSlice<T> for I {}

pub trait UnsafeParSlice<T>: PointerParSlice<T> {
    #[inline(always)]
    unsafe fn get(&self, index: usize) -> &T {
        unsafe {
            // Safety: the caller must guarantee not to modify the memory pointed
            // by this reference for the duration of its lifetime and not to create
            // a &mut reference with `get_mut`
            &*self.get_ptr(index)
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
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut T {
        unsafe {
            // Safety: the caller must guarantee that no other references with the same index
            // exists for the duration of the returned reference
            &mut *self.get_mut_ptr_unchecked(index)
        }
    }
}

impl<T, I: PointerParSlice<T>> UnsafeParSlice<T> for I {}
