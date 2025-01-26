use crate::*;
use std::fmt::Debug;

/// Utility struct for contructors for slices that allow unsynchronized access
/// to their elements through [`UnsafeAccess`] and [`UnsafeChunkAccess`].
pub struct UnsafeParSlice;

impl UnsafeParSlice {
    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to its elements through [`UnsafeAccess`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let unsafe_slice = UnsafeParSlice::new(4);
    ///
    /// unsafe {
    ///     *unsafe_slice.get_mut(0) = 42;
    /// }
    ///
    /// assert_eq!(unsafe_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[allow(clippy::new_ret_no_self)]
    #[inline(always)]
    pub fn new<T: Default + Send + Sync>(
        len: usize,
    ) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice(len).into_unsafe_par_slice()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to its elements through [`UnsafeAccess`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let unsafe_slice = UnsafeParSlice::with_value(69, 4);
    ///
    /// unsafe {
    ///     *unsafe_slice.get_mut(0) = 42;
    /// }
    ///
    /// assert_eq!(unsafe_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline(always)]
    pub fn with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
    ) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice_with_value(len, value).into_unsafe_par_slice()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to its elements through [`UnsafeAccess`] and that can be
    /// converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let unsafe_slice = UnsafeParSlice::with_closure(|i| i, 4);
    ///
    /// unsafe {
    ///     *unsafe_slice.get_mut(0) = 42;
    /// }
    ///
    /// assert_eq!(unsafe_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline(always)]
    pub fn with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
    ) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice_with(len, closure).into_unsafe_par_slice()
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to [`T::default`](`Default::default`), that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeChunkAccess`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let unsafe_slice = UnsafeParSlice::new_chunks(4, 2);
    ///
    /// unsafe {
    ///     unsafe_slice.get_mut(0)[0] = 42;
    /// }
    ///
    /// assert_eq!(unsafe_slice.into().as_ref(), &[42, 0, 0, 0]);
    /// ```
    #[inline(always)]
    pub fn new_chunks<T: Default + Send + Sync>(
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice(len).into_unsafe_par_chunk_slice(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to `value`, that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeChunkAccess`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let unsafe_slice = UnsafeParSlice::chunks_with_value(69, 4, 2);
    ///
    /// unsafe {
    ///     unsafe_slice.get_mut(0)[0] = 42;
    /// }
    ///
    /// assert_eq!(unsafe_slice.into().as_ref(), &[42, 69, 69, 69]);
    /// ```
    #[inline(always)]
    pub fn chunks_with_value<T: Clone + Send + Sync>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with_value(len, value).into_unsafe_par_chunk_slice(chunk_size)
    }

    /// Constructs a new slice with `len` elements, each initialized
    /// to the return value of `closure` called with the index of the element
    /// to generate as an [`usize`], that allows unsynchronized
    /// access to chunks of `chunk_size` of its elements through
    /// [`UnsafeChunkAccess`] and that can be converted into a boxed slice.
    ///
    /// # Examples
    /// ```
    /// # use par_slice::*;
    /// let unsafe_slice = UnsafeParSlice::chunks_with_closure(|i| i, 4, 2);
    ///
    /// unsafe {
    ///     unsafe_slice.get_mut(0)[0] = 42;
    /// }
    ///
    /// assert_eq!(unsafe_slice.into().as_ref(), &[42, 1, 2, 3]);
    /// ```
    #[inline(always)]
    pub fn chunks_with_closure<T: Send + Sync>(
        closure: impl FnMut(usize) -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with(len, closure).into_unsafe_par_chunk_slice(chunk_size)
    }
}
