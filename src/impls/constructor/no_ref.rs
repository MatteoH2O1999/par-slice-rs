use crate::*;

/// Utility struct for contructors for slices that allow unsynchronized access
/// to their elements through [`UnsafeNoRefIndex`] and [`UnsafeNoRefChunkIndex`].
pub struct NoRefParSlice;

impl NoRefParSlice {
    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to its elements through [`UnsafeNoRefIndex`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = NoRefParSlice::new(4);
    ///
    /// unsafe {
    ///     data_race_slice.set_value(0, 42);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new<T: Default + Send + Sync>(
        len: usize,
    ) -> impl UnsafeNoRefIndex<T> + ParCollection<T, Box<[T]>> {
        new_boxed_slice(len).into_par_index_no_ref()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to its elements through [`UnsafeNoRefIndex`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = NoRefParSlice::with_value(69, 4);
    ///
    /// unsafe {
    ///     data_race_slice.set_value(0, 42);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline]
    pub fn with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
    ) -> impl UnsafeNoRefIndex<T> + ParCollection<T, Box<[T]>> {
        new_boxed_slice_with_value(len, value).into_par_index_no_ref()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to its elements through [`UnsafeNoRefIndex`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = NoRefParSlice::with_closure(|i| i, 4);
    ///
    /// unsafe {
    ///     data_race_slice.set_value(0, 42);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline]
    pub fn with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
    ) -> impl UnsafeNoRefIndex<T> + ParCollection<T, Box<[T]>> {
        new_boxed_slice_with(len, closure).into_par_index_no_ref()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeNoRefChunkIndex`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = NoRefParSlice::new_chunks(4, 2);
    ///
    /// unsafe {
    ///     data_race_slice.set_values(0, &[42, 0]);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[inline]
    pub fn new_chunks<T: Default + Send + Sync>(
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<[T], Box<[T]>> {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice(len).into_par_chunk_index_no_ref(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeNoRefChunkIndex`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = NoRefParSlice::chunks_with_value(69, 4, 2);
    ///
    /// unsafe {
    ///     data_race_slice.set_values(0, &[42, 69]);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline]
    pub fn chunks_with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<[T], Box<[T]>> {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with_value(len, value).into_par_chunk_index_no_ref(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeNoRefChunkIndex`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let data_race_slice = NoRefParSlice::chunks_with_closure(|i| i, 4, 2);
    ///
    /// unsafe {
    ///     data_race_slice.set_values(0, &[42, 1]);
    /// }
    ///
    /// assert_eq!(data_race_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline]
    pub fn chunks_with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<[T], Box<[T]>> {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with(len, closure).into_par_chunk_index_no_ref(chunk_size)
    }
}
