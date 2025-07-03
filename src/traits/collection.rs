use std::fmt::Debug;

/// A sized collection.
///
/// This trait can be trusted by unsafe code thanks to the invariants below.
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * The collection must hold a number of elements equal to [`len`](`TrustedSizedCollection::len`).
/// * [`is_empty`](`TrustedSizedCollection::is_empty`) must return `true` if and only if `len == 0`
///   (in other words: `collection.is_empty() == (collection.len() == 0)`).
pub unsafe trait TrustedSizedCollection {
    /// Returns the number of elements in the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index();
    /// assert_eq!(collection.len(), 5);
    /// ```
    fn len(&self) -> usize;

    /// Returns `true` if the collection has no elements in it.
    ///
    /// Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let mut v = Vec::new();
    /// {
    ///     let collection = v.as_par_index();
    ///     assert!(collection.is_empty());
    /// }
    /// v.push(42);
    /// {
    ///     let collection = v.as_par_index();
    ///     assert!(!collection.is_empty());
    /// }
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A sized collection that can be used in chunks of equal size.
///
/// This trait can be trusted by unsafe code thanks to the invariants below.
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * Each chunk must have the same size equal to [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`).
/// * The collection holds a number of chunks equal to [`num_chunks`](`TrustedChunkSizedCollection::num_chunks`).
/// * The [`len`](`TrustedSizedCollection::len`) method must be an alias to [`num_chunks`](`TrustedChunkSizedCollection::num_chunks`)
///   (it must hold `collection.len() == collection.num_chunks()`).
/// * The collection must hold a number of elements equal to [`num_elements`](`TrustedChunkSizedCollection::num_elements`).
/// * The number of elements in the collection is equal to the number of chunks in the collection times the chunk size
///   (in other words: `num_elements = num_chunks * chunk_size`).
pub unsafe trait TrustedChunkSizedCollection: TrustedSizedCollection {
    /// Returns the number of elements in each chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 20].into_par_chunk_index(5);
    /// assert_eq!(collection.chunk_size(), 5);
    /// ```
    fn chunk_size(&self) -> usize;

    /// Returns the number of elements in the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 20].into_par_chunk_index(5);
    /// assert_eq!(collection.num_elements(), 20);
    /// ```
    #[inline]
    fn num_elements(&self) -> usize {
        self.num_chunks() * self.chunk_size()
    }

    /// Returns the number of chunks in the collection.
    ///
    /// This is equivalent to [`len`](`TrustedSizedCollection::len`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 20].into_par_chunk_index(5);
    /// assert_eq!(collection.num_chunks(), 4);
    /// assert_eq!(collection.len(), 4);
    /// ```
    #[inline]
    fn num_chunks(&self) -> usize {
        self.len()
    }
}

/// Traits common to parallel views on collections.
///
/// `T` is the type of the collection's elements.
pub trait ParView<T: ?Sized>: Send + Sync + Debug {}

impl<T: ?Sized, G: Send + Sync + Debug> ParView<T> for G {}

/// Traits common to parallel collections.
///
/// `T` is the type of the collection's elements, `C` is the wrapped collection.
pub trait ParCollection<T: ?Sized, C>: ParView<T> + Into<C> {}

impl<T: ?Sized, C, G: Into<C> + ParView<T>> ParCollection<T, C> for G {}

/// Asserts that `index` is between `0` and `len - 1`, panicking otherwise.
#[inline]
pub(crate) fn assert_in_bounds(len: usize, index: usize) {
    assert!(index < len, "Index {index} invalid for slice of len {len}")
}

/// Asserts that `chunk.len()` is equal to `chunk_size`, panicking otherwise
#[inline]
pub(crate) fn assert_chunk_compatible<T>(chunk_size: usize, chunk: &[T]) {
    assert!(
        chunk.len() == chunk_size,
        "value should have the same length as the chunk. Got a value of length {} for a chunk of length {}",
        chunk.len(),
        chunk_size
    )
}

/// Asserts that a collection of size `len` can be split exactly in chunks of size `chunk_size`,
/// panicking if this is not true.
#[inline]
pub(crate) fn assert_chunk_size(len: usize, chunk_size: usize) {
    assert!(
        len % chunk_size == 0,
        "chunk_size should be a divisor of len. {} / {} = {} with a remainder of {}",
        len,
        chunk_size,
        len / chunk_size,
        len % chunk_size
    )
}
