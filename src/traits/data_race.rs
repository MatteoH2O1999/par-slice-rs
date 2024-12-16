use super::*;

pub trait UnsafeDataRaceParSlice<T: ?Sized>: PointerParSlice<T> {
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
    unsafe fn get_clone(&self, index: usize) -> T
    where
        T: Clone,
    {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            (*self.get_ptr(index)).clone()
        }
    }

    #[inline(always)]
    unsafe fn get_clone_unchecked(&self, index: usize) -> T
    where
        T: Clone,
    {
        unsafe {
            // Safety: the caller must guarantee that there are no data races
            (*self.get_ptr_unchecked(index)).clone()
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

pub trait UnsafeDataRaceParChunkSlice<T>: PointerParSlice<[T]> {
    #[inline(always)]
    unsafe fn get(&self, index: usize) -> Box<[T]>
    where
        T: Clone,
    {
        let fat_ptr = self.get_ptr(index);

        let mut res = Box::new_uninit_slice(fat_ptr.len());
        let mut ptr = fat_ptr as *const T;

        for elem in res.iter_mut() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                elem.write((*ptr).clone());

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
        T: Clone,
    {
        let fat_ptr = self.get_ptr_unchecked(index);

        let mut res = Box::new_uninit_slice(fat_ptr.len());
        let mut ptr = fat_ptr as *const T;

        for elem in res.iter_mut() {
            unsafe {
                // Safety: the caller must guarantee that there are no data races
                elem.write((*ptr).clone());

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
