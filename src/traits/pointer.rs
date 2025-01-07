use crate::*;

/// Unsynchronized access to elements of a collection through pointers.
///
/// The trait allows *unsynchronized* access to the elements of a collection by
/// allowing the creation of *mutable pointers* from a *shared reference* to the
/// collection and its index.
///
/// The user is responsible to avoid data races and to respect Rust's *aliasing rules* (one or more shared references or
/// exactly one mutable reference) when dereferencing pointers.
///
/// For more details see the individual methods.
///
/// # Safety
///
/// Implementors must be careful of [undefined behavior] when returning mutable pointers
/// starting from a shared reference.
/// It is advisable to learn about [interior mutability](https://doc.rust-lang.org/reference/interior-mutability.html)
/// before trying to implement this trait.
///
/// In particular, implementors must guarantee that ponters are valid, dereferenceable and do not alias or overlap as long as
/// different indexes are used for the trait's methods.
/// In addition, the following invariants must hold:
/// * The collection has size [`len`](`TrustedSizedCollection::len`).
/// * For each collection of size `n`, indexes are defined from `0` to `n - 1`, each univocally identifying an element in
///   the collection.
/// * For each index `i`, `collection.get_ptr(i)` returns an immutable pointer to the element identified by index `i` in the
///   collection, panicking whenever `i` is out of bounds. It is still up to the caller to avoid data races and to respect
///   Rust's *aliasing rules* when dereferencing the pointer.
/// * For each index `i`, `collection.get_ptr_unchecked(i)` returns an immutable pointer to the element identified by index `i`
///   in the collection. It is up to the caller to avoid data races and to respect Rust's *aliasing rules* when dereferencing
///   the pointer and to ensure that `i` is in bounds.
/// * For each index `i`, `collection.get_mut_ptr(i)` returns a mutable pointer to the element identified by index `i` in the
///   collection, panicking whenever `i` is out of bounds. It is still up to the caller to avoid data races and to respect
///   Rust's *aliasing rules* when dereferencing the pointer.
/// * For each index `i`, `collection.get_mut_ptr_unchecked(i)` returns a mutable pointer to the element identified by index `i`
///   in the collection. It is up to the caller to avoid data races and to respect Rust's *aliasing rules* when dereferencing
///   the pointer and to ensure that `i` is in bounds.
/// * For each valid index `i`, `collection.get_ptr(i) == collection.get_ptr_unchecked(i)`.
/// * For each valid index `i`, `collection.get_mut_ptr(i) == collection.get_mut_ptr_unchecked(i)`.
/// * Every returned pointer must be valid for the lifetime of the collection.
///
/// # Examples
///
/// We can create all the pointers we want:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_pointer_par_slice();
/// let mut_ptr_0 = collection.get_mut_ptr(0);
/// let mut_ptr_1 = unsafe {
///     // We know 1 is a valid index
///     collection.get_mut_ptr_unchecked(1)
/// };
/// let ptr_0 = collection.get_ptr(1);
/// let ptr_1 = unsafe {
///     // We know 1 is a valid index
///     collection.get_ptr_unchecked(1)
/// };
/// ```
///
/// In order to dereference pointers we must ensure no data races can happen:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_pointer_par_slice();
/// let ptr = collection.get_mut_ptr(0);
/// unsafe {
///     // There are no data races and no references to element 0
///     // so this is safe.
///     *ptr = 42;
/// }
/// assert_eq!(collection.into().as_ref(), vec![42, 0, 0, 0, 0]);
/// ```
///
/// We can also create references if we can guarantee Rust's aliasing rules:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_pointer_par_slice();
/// let ptr = collection.get_mut_ptr(0);
/// {
///     let reference = unsafe {
///         // No other references to element 0 exist so this is safe.
///         &mut *ptr
///     };
///     *reference = 42;
/// }
/// assert_eq!(collection.into().as_ref(), vec![42, 0, 0, 0, 0]);
/// ```
///
/// This is undefined behavior:
///
/// ```no_run
/// # use par_slice::*;
/// let collection = vec![0; 5].into_pointer_par_slice();
/// let ptr = collection.get_mut_ptr(0);
/// {
///     let reference = unsafe {
///         // No other references to element 0 exist so this is safe.
///         &mut *ptr
///     };
///     *reference = 42;
///     let reference_copy = unsafe {
///         // This is UB: reference is still alive
///         & *ptr
///     };
/// }
/// ```
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub unsafe trait PointerAccess<T: ?Sized>: TrustedSizedCollection {
    /// Returns an immutable pointer to the element identified by `index` in the collection, without performing
    /// bounds checking.
    ///
    /// This method does not perform bounds checking on `index` to ensure its validity.
    /// If you can't ensure its validity, you may want to use the [`get_ptr`](`Self::get_ptr`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method with an index `i` that would panic [`get_ptr`](`Self::get_ptr`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_pointer_par_slice();
    /// // We know 0 is a valid index for a collection of length 5
    /// let ptr_0: *const usize = unsafe { collection.get_ptr_unchecked(0) };
    /// assert_eq!(unsafe {*ptr_0}, 0);
    /// ```
    unsafe fn get_ptr_unchecked(&self, index: usize) -> *const T;

    /// Returns a mutable pointer to the element identified by `index` in the collection, without performing
    /// bounds checking.
    ///
    /// This method does not performs bounds checking on `index` to ensure its validity.
    /// If you can't ensure its validity, you may want to use the [`get_mut_ptr`](`Self::get_mut_ptr`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method with an index `i` that would panic [`get_mut_ptr`](`Self::get_mut_ptr`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_pointer_par_slice();
    /// // We know 0 is a valid index for a collection of length 5
    /// let ptr_0: *mut usize = unsafe { collection.get_mut_ptr_unchecked(0) };
    /// // No other reference exists so we may dereference ptr_0 safely
    /// unsafe { *ptr_0 = 42 };
    /// assert_eq!(unsafe { *collection.get_ptr(0) }, 42);
    /// ```
    unsafe fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T;

    /// Returns an immutable pointer to the element identified by `index` in the collection.
    ///
    /// This method performs bounds checking on `index` to ensure its validity.
    /// If you can ensure its validity, you may want to use the [`get_ptr_unchecked`](`Self::get_ptr_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_pointer_par_slice();
    /// let ptr_0: *const usize =  collection.get_ptr(0);
    /// // No other reference exists so we may dereference ptr_0 safely
    /// assert_eq!(unsafe { *ptr_0 }, 0);
    /// ```
    #[inline(always)]
    fn get_ptr(&self, index: usize) -> *const T {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_ptr_unchecked(index)
        }
    }

    /// Returns a mutable reference to the element identified by `index` in the collection.
    ///
    /// This method performs bounds checking on `index` to ensure its validity.
    /// If you can ensure its validity, you may want to use the [`get_mut_ptr_unchecked`](`Self::get_mut_ptr_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_pointer_par_slice();
    /// let ptr_0: *mut usize = collection.get_mut_ptr(0);
    /// // No other reference exists so we may dereference ptr_0 safely
    /// unsafe { *ptr_0 = 42 };
    /// assert_eq!(unsafe { *collection.get_ptr(0) }, 42);
    /// ```
    #[inline(always)]
    fn get_mut_ptr(&self, index: usize) -> *mut T {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_mut_ptr_unchecked(index)
        }
    }
}

/// Marker trait for collections that allow unsynchronized access to non-overlapping chunks of their elements through pointers.
///
/// The trait allows *unsynchronized* access to chunks of elements of a collection by
/// allowing the creation of *mutable pointers* to chunks of elements from a *shared reference* to the
/// collection and its index.
///
/// The user is responsible to avoid data races and to respect Rust's *aliasing rules* (one or more shared references or
/// exactly one mutable reference) when dereferencing pointers.
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * The collection contains [`num_chunks`](`TrustedChunkSizedCollection::num_chunks`) **non-overlapping** slices of `T`,
///   each of len [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`).
/// * For each collection of size `n`, chunk indexes are defined from `0` to `n - 1`, each univocally identifying a chunk of elements in
///   the collection as follows: a chunk of index `i` includes all elements from index `i * collection.chunk_size()` included to
///   `(i + 1) * collection.chunk_size()` excluded.
/// * The collection implements [`UnsafeAccess<[T]>`](`UnsafeAccess`) where `[T]` is a chunk, so `[T].len() == collection.chunk_size()`,
///   and where all the methods' indexes refer to the chunk indexes as defined above.
pub unsafe trait PointerChunkAccess<T>:
    PointerAccess<[T]> + TrustedChunkSizedCollection
{
}
