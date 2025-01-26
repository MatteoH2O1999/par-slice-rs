use crate::*;
use std::fmt::Debug;

/// Utility struct for contructors for slices that allow unsynchronized access
/// to their elements through [`UnsafeDataRaceAccess`] and [`UnsafeDataRaceChunkAccess`].
pub struct DataRaceParSlice;

impl DataRaceParSlice {
    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to its elements through [`UnsafeDataRaceAccess`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = DataRaceParSlice::new(4);
    ///
    /// unsafe {
    ///     data_race_slice.set(0, 42);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[allow(clippy::new_ret_no_self)]
    #[inline(always)]
    pub fn new<T: Default + Send + Sync>(
        len: usize,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice(len).into_data_race_par_slice()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to its elements through [`UnsafeDataRaceAccess`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = DataRaceParSlice::with_value(69, 4);
    ///
    /// unsafe {
    ///     data_race_slice.set(0, 42);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline(always)]
    pub fn with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice_with_value(len, value).into_data_race_par_slice()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to its elements through [`UnsafeDataRaceAccess`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = DataRaceParSlice::with_closure(|i| i, 4);
    ///
    /// unsafe {
    ///     data_race_slice.set(0, 42);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline(always)]
    pub fn with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
    ) -> impl UnsafeDataRaceAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice_with(len, closure).into_data_race_par_slice()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeDataRaceChunkAccess`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = DataRaceParSlice::new_chunks(4, 2);
    ///
    /// unsafe {
    ///     data_race_slice.set(0, &[42, 0]);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[inline(always)]
    pub fn new_chunks<T: Default + Send + Sync>(
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice(len).into_data_race_par_chunk_slice(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeDataRaceChunkAccess`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = DataRaceParSlice::chunks_with_value(69, 4, 2);
    ///
    /// unsafe {
    ///     data_race_slice.set(0, &[42, 69]);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline(always)]
    pub fn chunks_with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with_value(len, value).into_data_race_par_chunk_slice(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeDataRaceChunkAccess`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = DataRaceParSlice::chunks_with_closure(|i| i, 4, 2);
    ///
    /// unsafe {
    ///     data_race_slice.set(0, &[42, 1]);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline(always)]
    pub fn chunks_with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with(len, closure).into_data_race_par_chunk_slice(chunk_size)
    }
}
