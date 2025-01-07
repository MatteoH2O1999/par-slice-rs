use crate::*;
use std::fmt::Debug;

pub unsafe trait ParSliceView<T> {
    fn as_pointer_par_slice(&mut self) -> impl PointerAccess<T> + Sync + Debug;

    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceAccess<T> + Sync + Debug;

    fn as_unsafe_par_slice(&mut self) -> impl UnsafeAccess<T> + Sync + Debug;

    fn as_pointer_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Sync + Debug;

    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Sync + Debug;

    fn as_unsafe_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Sync + Debug;
}

pub unsafe trait IntoParSlice<T> {
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync + Debug;

    fn into_data_race_par_slice(
        self,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync + Debug;

    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync + Debug;

    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Into<Box<[T]>> + Sync + Debug;

    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync + Debug;

    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Into<Box<[T]>> + Sync + Debug;
}
