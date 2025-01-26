use crate::*;

unsafe impl<T: Send + Sync> ParSliceView<T> for [T] {
    #[inline(always)]
    fn as_pointer_par_slice(&mut self) -> impl PointerAccess<T> + ParView {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceAccess<T> + ParView {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_unsafe_par_slice(&mut self) -> impl UnsafeAccess<T> + ParView {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_pointer_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + ParView {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + ParView {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_unsafe_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + ParView {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }
}

unsafe impl<T: Send + Sync> IntoParSlice<T> for Box<[T]> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceAccess<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }
}

unsafe impl<T: Send + Sync> IntoParSlice<T> for Vec<T> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline(always)]
    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceAccess<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }
}
