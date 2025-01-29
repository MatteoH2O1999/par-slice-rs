use crate::*;

/// Unsynchronized access to elements of a collection through references.
///
/// The trait allows *unsynchronized* access to the elements of a collection by
/// allowing the creation of *mutable references* from a *shared reference* to the
/// collection and its index.
///
/// The user is responsible to respect Rust's *aliasing rules* (one or more shared references or
/// exactly one mutable reference).
///
/// For more details see the individual methods.
///
/// # Safety
///
/// Implementors must be careful of [undefined behavior] when returning mutable references
/// starting from a shared reference.
/// It is advisable to learn about [interior mutability](https://doc.rust-lang.org/reference/interior-mutability.html)
/// before trying to implement this trait.
///
/// In particular, implementors must guarantee that references are valid and do not alias or overlap as long as
/// different indexes are used for the trait's methods.
/// In addition, the following invariants must hold:
/// * The collection has size [`len`](`TrustedSizedCollection::len`).
/// * For each collection of size `n`, indexes are defined from `0` to `n - 1`, each univocally identifying an element in
///   the collection.
/// * For each index `i`, `collection.get(i)` returns a shared reference to the element identified by index `i` in the collection,
///   panicking whenever `i` is out of bounds. It is still up to the caller to ensure Rust's aliasing rules
///   are respected.
/// * For each index `i`, `collection.get_unchecked(i)` returns a shared reference to the element identified by index `i` in the collection.
///   It is up to the caller to ensure Rust's aliasing rules are respected and that `i` is in bounds.
/// * For each index `i`, `collection.get_mut(i)` returns a mutable reference to the element identified by index `i` in the collection,
///   panicking whenever `i` is out of bounds. It is still up to the caller to ensure Rust's aliasing rules
///   are respected.
/// * For each index `i`, `collection.get_mut_unchecked(i)` returns a mutable reference to the element identified by index `i` in the collection.
///   It is up to the caller to ensure Rust's aliasing rules are respected and that `i` is in bounds.
/// * For each valid index `i`, `collection.get(i) == collection.get_unchecked(i)`.
/// * For each valid index `i`, `collection.get_mut(i) == collection.get_mut_unchecked(i)`.
///
/// # Examples
///
/// We can create multiple mutable references to different indexes as long as we respect Rust's aliasing rules:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_par_index();
///
/// unsafe {
///     // This checks 0 is a valid index
///     *collection.get_mut(0) = 42;
///     // We know 1 is a valid index for a vector of length 5
///     *collection.get_mut_unchecked(1) = 69;
/// }
///
/// assert_eq!(collection.into().as_ref(), vec![42, 69, 0, 0, 0]);
/// ```
///
/// Note how creating two mutable references to the same index is [undefined behavior]:
///
/// ```no_run
/// # use par_slice::*;
/// let collection = vec![0; 5].into_par_index();
///
/// unsafe {
///     let mut_ref_0 = collection.get_mut(0);
///     // Instant UB: Rust's aliasing rules were violated
///     let mut_ref_0_copy = collection.get_mut_unchecked(0);
/// }
/// ```
///
/// Multiple shared references to the same index are allowed:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_par_index();
///
/// unsafe {
///     let ref_0 = collection.get(0);
///     // OK: Rust's aliasing rules are not violated
///     let ref_0_copy = collection.get_unchecked(0);
///     
///     assert_eq!(*ref_0, 0);
///     assert_eq!(*ref_0_copy, 0);
/// }
/// ```
///
/// But creating a mutable reference when any other reference to the same element exists is not:
///
/// ```no_run
/// # use par_slice::*;
/// let collection = vec![0; 5].into_par_index();
///
/// unsafe {
///     let ref_0 = collection.get(0);
///     // Instant UB: Rust's aliasing rules were violated
///     let mut_ref_0 = collection.get_mut_unchecked(0);
/// }
///
/// // Equivalently
///
/// unsafe {
///     let mut_ref_0 = collection.get_mut(0);
///     // Instant UB: Rust's aliasing rules were violated
///     let ref_0 = collection.get_unchecked(0);
/// }
/// ```
///
/// In order to avoid this you must separate their lifetimes:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_par_index();
///
/// unsafe {
///     let ref_0 = collection.get(0);
/// }
/// unsafe {
///     // OK: ref_0 no longer exists
///     let mut_ref_0 = collection.get_mut_unchecked(0);
/// }
///
/// // Equivalently
///
/// unsafe {
///     let mut_ref_0 = collection.get_mut(0);
/// }
/// unsafe {
///     // OK: mut_ref_0 no longer exists
///     let ref_0 = collection.get_unchecked(0);
/// }
/// ```
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub unsafe trait UnsafeIndex<T: ?Sized>: TrustedSizedCollection {
    /// Returns a shared reference to the element identified by `index` in the collection.
    ///
    /// This method performs bounds checking on `index` to ensure its validity.
    /// If you can ensure its validity, you may want to use the [`get_unchecked`](`Self::get_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection.
    ///
    /// # Safety
    ///
    /// Calling this method while a mutable reference to the same element still exists is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index();
    /// let ref_0: &usize = unsafe { collection.get(0) };
    /// assert_eq!(*ref_0, 0);
    /// ```
    unsafe fn get(&self, index: usize) -> &T {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_unchecked(index)
        }
    }

    /// Returns a shared reference to the element identified by `index` in the collection, without performing
    /// bounds checking.
    ///
    /// This method does not perform bounds checking on `index` to ensure its validity.
    /// If you can't ensure its validity, you may want to use the [`get`](`Self::get`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while a mutable reference to the same element still exists is undefined behavior.
    /// Calling this method with an index `i` that would panic [`get`](`Self::get`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index();
    /// // We know 0 is a valid index for a collection of length 5
    /// let ref_0: &usize = unsafe { collection.get_unchecked(0) };
    /// assert_eq!(*ref_0, 0);
    /// ```
    unsafe fn get_unchecked(&self, index: usize) -> &T;

    /// Returns a mutable reference to the element identified by `index` in the collection.
    ///
    /// This method performs bounds checking on `index` to ensure its validity.
    /// If you can ensure its validity, you may want to use the [`get_mut_unchecked`](`Self::get_mut_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection.
    ///
    /// # Safety
    ///
    /// Calling this method while a reference of any kind to the same element still exists is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index();
    /// {
    ///     let ref_0: &mut usize = unsafe { collection.get_mut(0) };
    ///     *ref_0 = 42;
    /// }
    /// // ref_0 is no longer in scope: we can create a shared reference to the same element
    /// assert_eq!(unsafe { *collection.get(0) }, 42);
    /// ```
    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    unsafe fn get_mut(&self, index: usize) -> &mut T {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_mut_unchecked(index)
        }
    }

    /// Returns a mutable reference to the element identified by `index` in the collection, without performing
    /// bounds checking.
    ///
    /// This method does not performs bounds checking on `index` to ensure its validity.
    /// If you can't ensure its validity, you may want to use the [`get_mut`](`Self::get_mut`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while a reference of any kind to the same element still exists is undefined behavior.
    /// Calling this method with an index `i` that would panic [`get_mut`](`Self::get_mut`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index();
    /// {
    ///     // We know 0 is a valid index for a collection of length 5
    ///     let ref_0: &mut usize = unsafe { collection.get_mut_unchecked(0) };
    ///     *ref_0 = 42;
    /// }
    /// // ref_0 is no longer in scope: we can create a shared reference to the same element
    /// assert_eq!(unsafe { *collection.get(0) }, 42);
    /// ```
    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut T;
}

/// Marker trait for collections that allow unsynchronized access to non-overlapping chunks of their elements through references.
///
/// The trait allows *unsynchronized* access to chunks of elements of a collection by
/// allowing the creation of *mutable references* to chunks of elements from a *shared reference* to the
/// collection and its index.
///
/// The user is responsible to respect Rust's *aliasing rules* (one or more shared references or
/// exactly one mutable reference).
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * The collection contains [`num_chunks`](`TrustedChunkSizedCollection::num_chunks`) **non-overlapping** slices of `T`,
///   each of len [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`).
/// * For each collection of size `n`, chunk indexes are defined from `0` to `n - 1`, each univocally identifying a chunk of elements in
///   the collection as follows: a chunk of index `i` includes all elements from index `i * collection.chunk_size()` included to
///   `(i + 1) * collection.chunk_size()` excluded.
/// * The collection implements [`UnsafeIndex<[T]>`](`UnsafeIndex`) where `[T]` is a chunk, so `[T].len() == collection.chunk_size()`,
///   and where all the methods' indexes refer to the chunk indexes as defined above.
pub unsafe trait UnsafeChunkIndex<T>:
    UnsafeIndex<[T]> + TrustedChunkSizedCollection
{
}
