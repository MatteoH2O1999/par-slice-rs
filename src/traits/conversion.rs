use crate::*;

pub trait ParSliceView<T> {
    fn as_pointer_par_slice(&mut self) -> impl PointerParSlice<T>;

    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceParSlice<T>;

    fn as_unsafe_par_slice(&mut self) -> impl UnsafeParSlice<T>;
}
