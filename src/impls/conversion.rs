use crate::*;

impl<T: Sync> ParSliceView<T> for [T] {
    #[inline(always)]
    fn as_pointer_par_slice(&mut self) -> impl PointerParSlice<T> + Sync {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceParSlice<T> + Sync {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_unsafe_par_slice(&mut self) -> impl UnsafeParSlice<T> + Sync {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_pointer_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerParSlice<[T]> + Sync {
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Sync {
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_unsafe_par_chunk_slice(&mut self, chunk_size: usize) -> impl UnsafeParSlice<[T]> + Sync {
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }
}

impl<T: Sync> IntoParSlice<T> for Box<[T]> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerParSlice<T> + Into<Box<[T]>> + Sync {
        let ptr = Box::into_raw(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellSlice::new_owned(ptr)
        }
    }

    #[inline(always)]
    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceParSlice<T> + Into<Box<[T]>> + Sync {
        let ptr = Box::into_raw(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellSlice::new_owned(ptr)
        }
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeParSlice<T> + Into<Box<[T]>> + Sync {
        let ptr = Box::into_raw(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellSlice::new_owned(ptr)
        }
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerParSlice<[T]> + Into<Box<[T]>> + Sync {
        let ptr = Box::into_raw(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellChunkSlice::new_owned(ptr, chunk_size)
        }
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Into<Box<[T]>> + Sync {
        let ptr = Box::into_raw(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellChunkSlice::new_owned(ptr, chunk_size)
        }
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeParSlice<[T]> + Into<Box<[T]>> + Sync {
        let ptr = Box::into_raw(self);
        unsafe {
            // Safety: pointer is now owned
            UnsafeCellChunkSlice::new_owned(ptr, chunk_size)
        }
    }
}

impl<T: Sync> IntoParSlice<T> for Vec<T> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerParSlice<T> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice().into_pointer_par_slice()
    }

    #[inline(always)]
    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceParSlice<T> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice().into_data_race_par_slice()
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeParSlice<T> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice().into_unsafe_par_slice()
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerParSlice<[T]> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice()
            .into_pointer_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice()
            .into_data_race_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeParSlice<[T]> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice()
            .into_unsafe_par_chunk_slice(chunk_size)
    }
}