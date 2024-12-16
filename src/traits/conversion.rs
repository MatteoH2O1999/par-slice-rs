use super::*;

pub trait ParSliceView<T> {
    fn as_pointer_par_slice(&mut self) -> impl PointerParSlice<T> + Sync;

    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceParSlice<T> + Sync;

    fn as_unsafe_par_slice(&mut self) -> impl UnsafeParSlice<T> + Sync;

    fn as_pointer_par_chunk_slice(&mut self, chunk_size: usize)
        -> impl PointerParSlice<[T]> + Sync;

    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Sync;

    fn as_unsafe_par_chunk_slice(&mut self, chunk_size: usize) -> impl UnsafeParSlice<[T]> + Sync;
}

pub trait IntoParSlice<T> {
    fn into_pointer_par_slice(self) -> impl PointerParSlice<T> + Sync;

    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceParSlice<T> + Sync;

    fn into_unsafe_par_slice(self) -> impl UnsafeParSlice<T> + Sync;

    fn into_pointer_par_chunk_slice(self, chunk_size: usize) -> impl PointerParSlice<[T]> + Sync;

    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Sync;

    fn into_unsafe_par_chunk_slice(self, chunk_size: usize) -> impl UnsafeParSlice<[T]> + Sync;
}
