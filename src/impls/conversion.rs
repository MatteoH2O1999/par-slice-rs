use crate::*;

unsafe impl<T: Send + Sync> ParIndexView<T> for [T] {
    #[inline(always)]
    fn as_pointer_par_index(&mut self) -> impl PointerIndex<T> + ParView {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_par_index_no_ref(&mut self) -> impl UnsafeNoRefIndex<T> + ParView {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_par_index(&mut self) -> impl UnsafeIndex<T> + ParView {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_pointer_par_chunk_index(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParView {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_par_chunk_index_no_ref(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParView {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_par_chunk_index(&mut self, chunk_size: usize) -> impl UnsafeChunkIndex<T> + ParView {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }
}

unsafe impl<T: Send + Sync> IntoParIndex<T> for Box<[T]> {
    #[inline(always)]
    fn into_pointer_par_index(self) -> impl PointerIndex<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_par_index_no_ref(self) -> impl UnsafeNoRefIndex<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_par_index(self) -> impl UnsafeIndex<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_pointer_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_par_chunk_index_no_ref(
        self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkIndex<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }
}

unsafe impl<T: Send + Sync> IntoParIndex<T> for Vec<T> {
    #[inline(always)]
    fn into_pointer_par_index(self) -> impl PointerIndex<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline(always)]
    fn into_par_index_no_ref(self) -> impl UnsafeNoRefIndex<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline(always)]
    fn into_par_index(self) -> impl UnsafeIndex<T> + ParCollection<Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline(always)]
    fn into_pointer_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }

    #[inline(always)]
    fn into_par_chunk_index_no_ref(
        self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }

    #[inline(always)]
    fn into_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkIndex<T> + ParCollection<Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }
}
