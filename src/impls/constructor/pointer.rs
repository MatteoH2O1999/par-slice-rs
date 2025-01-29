use crate::*;

/// Utility struct for contructors for slices that allow unsynchronized access
/// to their elements through [`PointerIndex`] and [`PointerChunkIndex`].
pub struct PointerParSlice;

impl PointerParSlice {
    #[allow(clippy::new_ret_no_self)]
    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to its elements through [`PointerIndex`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let pointer_slice = PointerParSlice::new(4);
    ///
    /// unsafe {
    ///     *pointer_slice.get_mut_ptr(0) = 42;
    /// }
    ///
    /// assert_eq!(pointer_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[inline]
    pub fn new<T: Default + Send + Sync>(
        len: usize,
    ) -> impl PointerIndex<T> + ParCollection<Box<[T]>> {
        new_boxed_slice(len).into_pointer_par_index()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to its elements through [`PointerIndex`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let pointer_slice = PointerParSlice::with_value(69, 4);
    ///
    /// unsafe {
    ///     *pointer_slice.get_mut_ptr(0) = 42;
    /// }
    ///
    /// assert_eq!(pointer_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline]
    pub fn with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
    ) -> impl PointerIndex<T> + ParCollection<Box<[T]>> {
        new_boxed_slice_with_value(len, value).into_pointer_par_index()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to its elements through [`PointerIndex`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let pointer_slice = PointerParSlice::with_closure(|i| i, 4);
    ///
    /// unsafe {
    ///     *pointer_slice.get_mut_ptr(0) = 42;
    /// }
    ///
    /// assert_eq!(pointer_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline]
    pub fn with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
    ) -> impl PointerIndex<T> + ParCollection<Box<[T]>> {
        new_boxed_slice_with(len, closure).into_pointer_par_index()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`PointerChunkIndex`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let pointer_slice = PointerParSlice::new_chunks(4, 2);
    ///
    /// unsafe {
    ///     (*pointer_slice.get_mut_ptr(0))[0] = 42;
    /// }
    ///
    /// assert_eq!(pointer_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[inline]
    pub fn new_chunks<T: Default + Send + Sync>(
        len: usize,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<Box<[T]>> {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice(len).into_pointer_par_chunk_index(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`PointerChunkIndex`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let pointer_slice = PointerParSlice::chunks_with_value(69, 4, 2);
    ///
    /// unsafe {
    ///     (*pointer_slice.get_mut_ptr(0))[0] = 42;
    /// }
    ///
    /// assert_eq!(pointer_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline]
    pub fn chunks_with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<Box<[T]>> {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with_value(len, value).into_pointer_par_chunk_index(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`PointerChunkIndex`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let pointer_slice = PointerParSlice::chunks_with_closure(|i| i, 4, 2);
    ///
    /// unsafe {
    ///     (*pointer_slice.get_mut_ptr(0))[0] = 42;
    /// }
    ///
    /// assert_eq!(pointer_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline]
    pub fn chunks_with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<Box<[T]>> {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with(len, closure).into_pointer_par_chunk_index(chunk_size)
    }
}
