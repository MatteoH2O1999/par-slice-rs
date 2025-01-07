use crate::*;
use std::fmt::Debug;

unsafe impl<T: Sync> ParSliceView<T> for [T] {
    #[inline(always)]
    fn as_pointer_par_slice(&mut self) -> impl PointerAccess<T> + Sync + Debug {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceAccess<T> + Sync + Debug {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_unsafe_par_slice(&mut self) -> impl UnsafeAccess<T> + Sync + Debug {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_pointer_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_unsafe_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }
}

unsafe impl<T: Sync> IntoParSlice<T> for Box<[T]> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync + Debug {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_data_race_par_slice(
        self,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync + Debug {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync + Debug {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }
}

unsafe impl<T: Sync> IntoParSlice<T> for Vec<T> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync + Debug {
        self.into_boxed_slice().into_pointer_par_slice()
    }

    #[inline(always)]
    fn into_data_race_par_slice(
        self,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync + Debug {
        self.into_boxed_slice().into_data_race_par_slice()
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync + Debug {
        self.into_boxed_slice().into_unsafe_par_slice()
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        self.into_boxed_slice()
            .into_pointer_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        self.into_boxed_slice()
            .into_data_race_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(self.len(), chunk_size);
        self.into_boxed_slice()
            .into_unsafe_par_chunk_slice(chunk_size)
    }
}
