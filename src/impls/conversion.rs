use crate::*;

unsafe impl<T: Send + Sync> ParIndexView<T> for [T] {
    #[inline]
    fn as_pointer_par_index(&mut self) -> impl PointerIndex<T> + ParView<T> {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline]
    fn as_par_index_no_ref(&mut self) -> impl UnsafeNoRefIndex<T> + ParView<T> {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline]
    fn as_par_index(&mut self) -> impl UnsafeIndex<T> + ParView<T> {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline]
    fn as_pointer_par_chunk_index(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParView<[T]> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline]
    fn as_par_chunk_index_no_ref(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParView<[T]> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline]
    fn as_par_chunk_index(&mut self, chunk_size: usize) -> impl UnsafeChunkIndex<T> + ParView<[T]> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }
}

unsafe impl<T: Send + Sync> IntoParIndex<T> for Box<[T]> {
    #[inline]
    fn into_pointer_par_index(self) -> impl PointerIndex<T> + ParCollection<T, Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline]
    fn into_par_index_no_ref(self) -> impl UnsafeNoRefIndex<T> + ParCollection<T, Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline]
    fn into_par_index(self) -> impl UnsafeIndex<T> + ParCollection<T, Self> {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline]
    fn into_pointer_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<[T], Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline]
    fn into_par_chunk_index_no_ref(
        self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<[T], Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline]
    fn into_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkIndex<T> + ParCollection<[T], Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }
}

unsafe impl<T: Send + Sync> IntoParIndex<T> for Vec<T> {
    #[inline]
    fn into_pointer_par_index(self) -> impl PointerIndex<T> + ParCollection<T, Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline]
    fn into_par_index_no_ref(self) -> impl UnsafeNoRefIndex<T> + ParCollection<T, Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline]
    fn into_par_index(self) -> impl UnsafeIndex<T> + ParCollection<T, Self> {
        UnsafeCellSlice::new_owned(self.into_boxed_slice())
    }

    #[inline]
    fn into_pointer_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<[T], Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }

    #[inline]
    fn into_par_chunk_index_no_ref(
        self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<[T], Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }

    #[inline]
    fn into_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkIndex<T> + ParCollection<[T], Self> {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self.into_boxed_slice(), chunk_size)
    }
}
