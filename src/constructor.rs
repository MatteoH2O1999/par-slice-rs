use crate::*;

pub struct ParSlice;

impl ParSlice {
    //
    // Pointer slices
    //

    #[inline(always)]
    pub fn new_pointer_par_slice<T: Sync + Default>(
        len: usize,
    ) -> impl PointerParSlice<T> + Into<Box<[T]>> + Sync {
        Self::pointer_par_slice_with(T::default, len)
    }

    pub fn pointer_par_slice_with_value<T: Sync + Clone>(
        value: T,
        len: usize,
    ) -> impl PointerParSlice<T> + Into<Box<[T]>> + Sync {
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(value.clone());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_pointer_par_slice()
    }

    pub fn pointer_par_slice_with<T: Sync>(
        mut closure: impl FnMut() -> T,
        len: usize,
    ) -> impl PointerParSlice<T> + Into<Box<[T]>> + Sync {
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(closure());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_pointer_par_slice()
    }

    #[inline(always)]
    pub fn new_pointer_par_chunk_slice<T: Sync + Default>(
        len: usize,
        chunk_size: usize,
    ) -> impl PointerParSlice<[T]> + Into<Box<[T]>> + Sync {
        Self::pointer_par_chunk_slice_with(T::default, len, chunk_size)
    }

    pub fn pointer_par_chunk_slice_with_value<T: Sync + Clone>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl PointerParSlice<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(value.clone());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_pointer_par_chunk_slice(chunk_size)
    }

    pub fn pointer_par_chunk_slice_with<T: Sync>(
        mut closure: impl FnMut() -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl PointerParSlice<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(closure());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_pointer_par_chunk_slice(chunk_size)
    }

    //
    // Data race slices
    //

    #[inline(always)]
    pub fn new_data_race_par_slice<T: Sync + Default>(
        len: usize,
    ) -> impl UnsafeDataRaceParSlice<T> + Into<Box<[T]>> + Sync {
        Self::data_race_par_slice_with(T::default, len)
    }

    pub fn data_race_par_slice_with_value<T: Sync + Clone>(
        value: T,
        len: usize,
    ) -> impl UnsafeDataRaceParSlice<T> + Into<Box<[T]>> + Sync {
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(value.clone());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_data_race_par_slice()
    }

    pub fn data_race_par_slice_with<T: Sync>(
        mut closure: impl FnMut() -> T,
        len: usize,
    ) -> impl UnsafeDataRaceParSlice<T> + Into<Box<[T]>> + Sync {
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(closure());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_data_race_par_slice()
    }

    #[inline(always)]
    pub fn new_data_race_par_chunk_slice<T: Sync + Default>(
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Into<Box<[T]>> + Sync {
        Self::data_race_par_chunk_slice_with(T::default, len, chunk_size)
    }

    pub fn data_race_par_chunk_slice_with_value<T: Sync + Clone>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(value.clone());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_data_race_par_chunk_slice(chunk_size)
    }

    pub fn data_race_par_chunk_slice_with<T: Sync>(
        mut closure: impl FnMut() -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceParChunkSlice<T> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(closure());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_data_race_par_chunk_slice(chunk_size)
    }

    //
    // Unsafe slices
    //

    #[inline(always)]
    pub fn new_unsafe_par_slice<T: Sync + Default>(
        len: usize,
    ) -> impl UnsafeParSlice<T> + Into<Box<[T]>> + Sync {
        Self::unsafe_par_slice_with(T::default, len)
    }

    pub fn unsafe_par_slice_with_value<T: Sync + Clone>(
        value: T,
        len: usize,
    ) -> impl UnsafeParSlice<T> + Into<Box<[T]>> + Sync {
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(value.clone());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_unsafe_par_slice()
    }

    pub fn unsafe_par_slice_with<T: Sync>(
        mut closure: impl FnMut() -> T,
        len: usize,
    ) -> impl UnsafeParSlice<T> + Into<Box<[T]>> + Sync {
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(closure());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_unsafe_par_slice()
    }

    #[inline(always)]
    pub fn new_unsafe_par_chunk_slice<T: Sync + Default>(
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeParSlice<[T]> + Into<Box<[T]>> + Sync {
        Self::unsafe_par_chunk_slice_with(T::default, len, chunk_size)
    }

    pub fn unsafe_par_chunk_slice_with_value<T: Sync + Clone>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeParSlice<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(value.clone());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_unsafe_par_chunk_slice(chunk_size)
    }

    pub fn unsafe_par_chunk_slice_with<T: Sync>(
        mut closure: impl FnMut() -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeParSlice<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        let mut boxed = Box::new_uninit_slice(len);
        for elem in boxed.iter_mut() {
            elem.write(closure());
        }
        let boxed = unsafe { boxed.assume_init() };
        boxed.into_unsafe_par_chunk_slice(chunk_size)
    }
}
