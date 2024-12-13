use super::*;

pub trait ParSliceView<T> {
    fn as_pointer_par_slice(&mut self) -> impl PointerParSlice<T>;

    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceParSlice<T>;

    fn as_unsafe_par_slice(&mut self) -> impl UnsafeParSlice<T>;

    fn as_pointer_par_chunk_slice(&mut self, chunk_size: usize) -> impl PointerParSlice<[T]>;

    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T>;

    fn as_unsafe_par_chunk_slice(&mut self, chunk_size: usize) -> impl UnsafeParSlice<[T]>;
}
