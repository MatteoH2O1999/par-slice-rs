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
/// * For each collection of size `n`, indexes are defined from `0` to `n - 1`, each univocally identifying an element in
///   the collection.
/// * For each index `i`, `slice.get(i)` returns a shared reference to the element identified by index `i` in the collection,
///   panicking whenever `i` is invalid (*i.e.* out of bounds). It is still up to the caller to ensure Rust's aliasing rules
///   are respected.
/// * For each index `i`, `slice.get_unchecked(i)` returns a shared reference to the element identified by index `i` in the collection.
///   It is up to the caller to ensure Rust's aliasing rules are respected and that `i` is valid.
/// * For each index `i`, `slice.get_mut(i)` returns a mutable reference to the element identified by index `i` in the collection,
///   panicking whenever `i` is invalid (*i.e.* out of bounds). It is still up to the caller to ensure Rust's aliasing rules
///   are respected.
/// * For each index `i`, `slice.get_mut_unchecked(i)` returns a mutable reference to the element identified by index `i` in the collection.
///   It is up to the caller to ensure Rust's aliasing rules are respected and that `i` is valid.
/// * For each valid index `i`, `slice.get(i) == slice.get_unchecked(i)`.
/// * For each valid index `i`, `slice.get_mut(i) == slice.get_mut_unchecked(i)`.
///
/// # Examples
///
/// Let's take `collection`, a collection of 5 integers set to 0 that implements this trait and can be converted into a
/// boxed slice.
///
/// We can create multiple mutable references to different indexes as long as we respect Rust's aliasing rules:
///
/// ```
/// # use par_slice::*;
/// #
/// # let collection = vec![0; 5].into_unsafe_par_slice();
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
/// #
/// # let collection = vec![0; 5].into_unsafe_par_slice();
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
/// #
/// # let collection = vec![0; 5].into_unsafe_par_slice();
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
/// But creating a mutable references when an other reference exists is not:
///
/// ```no_run
/// # use par_slice::*;
/// #
/// # let collection = vec![0; 5].into_unsafe_par_slice();
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
/// #
/// # let collection = vec![0; 5].into_unsafe_par_slice();
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
pub unsafe trait UnsafeAccess<T: ?Sized> {
    /// Returns a shared reference to the element identified by `index` in the collection.
    ///
    /// This method performs runtime checks on `index` to ensure its validity.
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
    /// Let's take `collection` a collection of 5 `usize` set to 0 that implements this trait.
    ///
    /// ```
    /// # use par_slice::*;
    /// #
    /// # let collection = vec![0; 5].into_unsafe_par_slice();
    /// let ref_0: &usize = unsafe { collection.get(0) };
    /// assert_eq!(*ref_0, 0);
    /// ```
    unsafe fn get(&self, index: usize) -> &T;

    /// Returns a shared reference to the element identified by `index` in the collection.
    ///
    /// This method does not performs runtime checks on `index` to ensure its validity.
    /// If you can't ensure its validity, you may want to use the [`get`](`Self::get`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while a mutable reference to the same element still exists is undefined behavior.
    /// Calling this method with an index `i` that would panic [`get`](`Self::get`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// Let's take `collection` a collection of 5 `usize` set to 0 that implements this trait.
    ///
    /// ```
    /// # use par_slice::*;
    /// #
    /// # let collection = vec![0; 5].into_unsafe_par_slice();
    /// // We know 0 is a valid index for a collection of length 5
    /// let ref_0: &usize = unsafe { collection.get_unchecked(0) };
    /// assert_eq!(*ref_0, 0);
    /// ```
    unsafe fn get_unchecked(&self, index: usize) -> &T;

    /// Returns a mutable reference to the element identified by `index` in the collection.
    ///
    /// This method performs runtime checks on `index` to ensure its validity.
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
    /// Let's take `collection` a collection of 5 `usize` set to 0 that implements this trait.
    ///
    /// ```
    /// # use par_slice::*;
    /// #
    /// # let collection = vec![0; 5].into_unsafe_par_slice();
    /// {
    ///     let ref_0: &mut usize = unsafe { collection.get_mut(0) };
    ///     *ref_0 = 42;
    /// }
    /// // ref_0 is no longer in scope: we can create a shared reference to the same element
    /// assert_eq!(unsafe { *collection.get(0) }, 42);
    /// ```
    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut(&self, index: usize) -> &mut T;

    /// Returns a mutable reference to the element identified by `index` in the collection.
    ///
    /// This method does not performs runtime checks on `index` to ensure its validity.
    /// If you can't ensure its validity, you may want to use the [`get_mut`](`Self::get_mut`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while a reference of any kind to the same element still exists is undefined behavior.
    /// Calling this method with an index `i` that would panic [`get_mut`](`Self::get_mut`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// Let's take `collection` a collection of 5 `usize` set to 0 that implements this trait.
    ///
    /// ```
    /// # use par_slice::*;
    /// #
    /// # let collection = vec![0; 5].into_unsafe_par_slice();
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
