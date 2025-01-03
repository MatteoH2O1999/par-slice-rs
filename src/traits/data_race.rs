use crate::*;

pub unsafe trait UnsafeDataRaceAccess<T: ?Sized>: TrustedSizedCollection {
    unsafe fn get(&self, index: usize) -> T
    where
        T: Copy,
    {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_unchecked(index)
        }
    }

    unsafe fn get_unchecked(&self, index: usize) -> T
    where
        T: Copy;

    unsafe fn set(&self, index: usize, value: T)
    where
        T: Sized,
    {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.set_unchecked(index, value);
        }
    }

    unsafe fn set_unchecked(&self, index: usize, value: T)
    where
        T: Sized;
}

pub unsafe trait UnsafeDataRaceChunkAccess<T>: TrustedChunkSizedCollection {
    #[inline(always)]
    unsafe fn get(&self, index: usize) -> Box<[T]>
    where
        T: Copy,
    {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_unchecked(index)
        }
    }

    unsafe fn get_unchecked(&self, index: usize) -> Box<[T]>
    where
        T: Copy;

    #[inline(always)]
    unsafe fn set(&self, index: usize, value: &[T])
    where
        T: Clone,
    {
        assert_in_bounds(self.len(), index);
        assert_chunk_compatible(self.chunk_size(), value);
        unsafe {
            // Safety: we just checked that index is in bounds and value is compatible
            // with chunk_size
            self.set_unchecked(index, value);
        }
    }

    unsafe fn set_unchecked(&self, index: usize, value: &[T])
    where
        T: Clone;
}
