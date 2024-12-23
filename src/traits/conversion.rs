use super::*;

pub unsafe trait ParSliceView<T> {
    fn as_pointer_par_slice(&mut self) -> impl PointerAccess<T> + Sync;

    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceAccess<T> + Sync;

    fn as_unsafe_par_slice(&mut self) -> impl UnsafeAccess<T> + Sync;

    fn as_pointer_par_chunk_slice(&mut self, chunk_size: usize) -> impl PointerAccess<[T]> + Sync;

    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Sync;

    fn as_unsafe_par_chunk_slice(&mut self, chunk_size: usize) -> impl UnsafeAccess<[T]> + Sync;
}

pub unsafe trait IntoParSlice<T> {
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync;

    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync;

    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync;

    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerAccess<[T]> + Into<Box<[T]>> + Sync;

    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync;

    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeAccess<[T]> + Into<Box<[T]>> + Sync;
}
