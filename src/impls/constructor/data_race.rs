use crate::{constructor::*, *};

pub struct DataRaceSlice;

impl DataRaceSlice {
    #[allow(clippy::new_ret_no_self)]
    #[inline(always)]
    pub fn new<T: Default + Sync>(
        len: usize,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync {
        new_boxed_slice(len).into_data_race_par_slice()
    }

    #[inline(always)]
    pub fn with_value<T: Clone + Sync>(
        value: T,
        len: usize,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync {
        new_boxed_slice_with_value(len, value).into_data_race_par_slice()
    }

    #[inline(always)]
    pub fn with_closure<T: Sync>(
        closure: impl FnMut() -> T,
        len: usize,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync {
        new_boxed_slice_with(len, closure).into_data_race_par_slice()
    }

    #[inline(always)]
    pub fn new_chunks<T: Default + Sync>(
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice(len).into_data_race_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    pub fn chunks_with_value<T: Clone + Sync>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with_value(len, value).into_data_race_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    pub fn chunks_with_closure<T: Sync>(
        closure: impl FnMut() -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with(len, closure).into_data_race_par_chunk_slice(chunk_size)
    }
}
