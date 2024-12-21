use crate::*;

#[inline(always)]
pub(crate) fn assert_chunk_size(len: usize, chunk_size: usize) {
    assert_eq!(
        len % chunk_size,
        0,
        "chunk_size should be a divisor of len. {} / {} = {} with a remainder of {}",
        len,
        chunk_size,
        len / chunk_size,
        len % chunk_size
    )
}

impl<T: Sync> ParSliceView<T> for [T] {
    #[inline(always)]
    fn as_pointer_par_slice(&mut self) -> impl PointerAccess<T> + Sync {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceAccess<T> + Sync {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_unsafe_par_slice(&mut self) -> impl UnsafeAccess<T> + Sync {
        UnsafeCellSlice::new_borrowed(self)
    }

    #[inline(always)]
    fn as_pointer_par_chunk_slice(&mut self, chunk_size: usize) -> impl PointerAccess<[T]> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }

    #[inline(always)]
    fn as_unsafe_par_chunk_slice(&mut self, chunk_size: usize) -> impl UnsafeAccess<[T]> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_borrowed(self, chunk_size)
    }
}

impl<T: Sync> IntoParSlice<T> for Box<[T]> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync {
        UnsafeCellSlice::new_owned(self)
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerAccess<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeAccess<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        UnsafeCellChunkSlice::new_owned(self, chunk_size)
    }
}

impl<T: Sync> IntoParSlice<T> for Vec<T> {
    #[inline(always)]
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice().into_pointer_par_slice()
    }

    #[inline(always)]
    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice().into_data_race_par_slice()
    }

    #[inline(always)]
    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync {
        self.into_boxed_slice().into_unsafe_par_slice()
    }

    #[inline(always)]
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerAccess<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        self.into_boxed_slice()
            .into_pointer_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        self.into_boxed_slice()
            .into_data_race_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeAccess<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(self.len(), chunk_size);
        self.into_boxed_slice()
            .into_unsafe_par_chunk_slice(chunk_size)
    }
}
