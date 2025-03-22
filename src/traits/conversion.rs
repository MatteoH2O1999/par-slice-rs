use crate::*;

/// View of a collection that allows unsynchronized access to its elements.
///
/// This trait allows to temporarily opt-in to unsynchronized access to the elements of
/// the underlying collection, either one-by-one or in chunks of arbitrary size.
///
/// The different methods allow the user to choose the level of access to use:
/// * [`as_par_index_no_ref`](`Self::as_par_index_no_ref`) and
///   [`as_par_chunk_index_no_ref`](`Self::as_par_chunk_index_no_ref`) allow access
///   through [`UnsafeNoRefIndex`] and [`UnsafeNoRefChunkIndex`] respectively
///   (see module documentation for more information).
/// * [`as_pointer_par_index`](`Self::as_pointer_par_index`) and
///   [`as_pointer_par_chunk_index`](`Self::as_pointer_par_chunk_index`) allow access
///   through [`PointerIndex`] and [`PointerChunkIndex`] respectively
///   (see module documentation for more information).
/// * [`as_par_index`](`Self::as_par_index`) and
///   [`as_par_chunk_index`](`Self::as_par_chunk_index`) allow access
///   through [`UnsafeIndex`] and [`UnsafeChunkIndex`] respectively
///   (see module documentation for more information).
///
/// Unsafe code can rely on this trait behavior thanks to the invariants specified below.
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * [`as_par_index_no_ref`](`Self::as_par_index_no_ref`),
///   [`as_pointer_par_index`](`Self::as_pointer_par_index`) and
///   [`as_par_index`](`Self::as_par_index`) return views on the collection
///   such that their [`len`](`TrustedSizedCollection::len`) is the size of the
///   collection and that if index `i` refers to element `x` in the collection, it refers
///   to element `x` in the returned views as well.
/// * [`as_par_chunk_index_no_ref`](`Self::as_par_chunk_index_no_ref`),
///   [`as_pointer_par_chunk_index`](`Self::as_pointer_par_chunk_index`) and
///   [`as_par_chunk_index`](`Self::as_par_chunk_index`) panic if the
///   collection's size is not divisible by `chunk_size`.
/// * [`as_par_chunk_index_no_ref`](`Self::as_par_chunk_index_no_ref`),
///   [`as_pointer_par_chunk_index`](`Self::as_pointer_par_chunk_index`) and
///   [`as_par_chunk_index`](`Self::as_par_chunk_index`) return views on the
///   collection such that their [`num_elements`](`TrustedChunkSizedCollection::num_elements`)
///   is equal to the size of the collection,
///   [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`) is equal to the `chunk_size`
///   parameter passed to the method, [`len`](`TrustedSizedCollection::len`) is
///   equal to `num_elements / chunk_size` and chunk indices follow the collection's original
///   indices (*i.e.* chunk 0 of a collection of `chunk_size` 4 includes indices from 0 to 3,
///   chunk 1 includes indices from 4 to 7, etc.).
///
/// # Examples
///
/// We may opt-in to different access paradigms in different scopes, but never more than 1
/// at any given time:
///
/// ```
/// # use par_slice::*;
/// let mut collection = vec![0; 10];
///
/// {
///     // Let's use pointers to single elements
///     let view = collection.as_pointer_par_index();
///     let ptr_1 = view.get_mut_ptr(1);
///     unsafe {
///         *ptr_1 = 42;
///     }
/// }
///
/// assert_eq!(collection, vec![0, 42, 0, 0, 0, 0, 0, 0, 0, 0]);
///
/// {
///     // Let's use setters and getters to chunks of size 2
///     let view = collection.as_par_chunk_index_no_ref(2);
///     unsafe {
///         view.set_values(1, &[69, 69]);
///     }
/// }
///
/// assert_eq!(collection, vec![0, 42, 69, 69, 0, 0, 0, 0, 0, 0]);
///
/// {
///     // Let's use references to chunks of size 5
///     let view = collection.as_par_chunk_index(5);
///     let last_five = unsafe { view.get_mut(1) };
///     let mut i = 1;
///     for elem in last_five.iter_mut() {
///         *elem = i;
///         i += 1;
///     }
///     last_five[2] = 42;
/// }
///
/// assert_eq!(collection, vec![0, 42, 69, 69, 0, 1, 2, 42, 4, 5]);
/// ```
pub unsafe trait ParIndexView<T> {
    /// Returns a view of the collection that allows unsynchronized access to
    /// its elements through pointers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let mut collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// {
    ///     let view = collection.as_pointer_par_index();
    ///     let mut_ptr_1 = view.get_mut_ptr(1);
    ///     let mut_ptr_5 = view.get_mut_ptr(5);
    ///     let ptr_2 = view.get_ptr(2);
    ///     unsafe {
    ///         *mut_ptr_1 = 42;
    ///         *mut_ptr_5 = 69;
    ///         assert_eq!(*ptr_2, 2);
    ///     }
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_pointer_par_index(&mut self) -> impl PointerIndex<T> + ParView<T>;

    /// Returns a view of the collection that allows unsynchronized access to its
    /// elements through setters and getters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let mut collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// {
    ///     let view = collection.as_par_index_no_ref();
    ///     unsafe {
    ///         view.set_value(1, 42);
    ///         view.set_value(5, 69);
    ///         assert_eq!(view.get_value(2), 2);
    ///     }
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_par_index_no_ref(&mut self) -> impl UnsafeNoRefIndex<T> + ParView<T>;

    /// Returns a view of the collection that allows unsynchronized access to its
    /// elements through references.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let mut collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// {
    ///     let view = collection.as_par_index();
    ///     let mut_ref_1 = unsafe { view.get_mut(1) };
    ///     let mut_ref_5 = unsafe { view.get_mut(5) };
    ///     let ref_2 = unsafe { view.get(2) };
    ///     *mut_ref_1 = 42;
    ///     *mut_ref_5 = 69;
    ///     assert_eq!(*ref_2, 2);
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_par_index(&mut self) -> impl UnsafeIndex<T> + ParView<T>;

    /// Returns a view of the collection that allows unsynchronized access to
    /// chunks of `chunk_size` of its elements through pointers.
    ///
    /// # Panics
    ///
    /// Panics if the size of the collection is not divisible by `chunk_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let mut collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// {
    ///     let view = collection.as_pointer_par_chunk_index(5);
    ///     let first_five = view.get_mut_ptr(0);
    ///     let last_five = view.get_mut_ptr(1);
    ///     unsafe {
    ///         (*first_five)[1] = 42;
    ///         (*last_five)[0] = 69;
    ///         assert_eq!((*first_five)[2], 2);
    ///     }
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_pointer_par_chunk_index(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParView<[T]>;

    /// Returns a view of the collection that allows unsynchronized access to
    /// chunks of `chunk_size` of its elements through setters and getters.
    ///
    /// # Panics
    ///
    /// Panics if the size of the collection is not divisible by `chunk_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let mut collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// {
    ///     let view = collection.as_par_chunk_index_no_ref(5);
    ///     unsafe {
    ///         view.set_values(0, &[0, 42, 2, 3, 4]);
    ///         view.set_values(1, &[69, 6, 7, 8, 9]);
    ///         assert_eq!(view.get_values(1, vec![0; 5]), vec![69, 6, 7, 8, 9]);
    ///     }
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_par_chunk_index_no_ref(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParView<[T]>;

    /// Returns a view of the collection that allows unsynchronized access to
    /// chunks of `chunk_size` of its elements through references.
    ///
    /// # Panics
    ///
    /// Panics if the size of the collection is not divisible by `chunk_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let mut collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// {
    ///     let view = collection.as_par_chunk_index(5);
    ///     let first_five = unsafe { view.get_mut(0) };
    ///     let last_five = unsafe { view.get_mut(1) };
    ///     first_five[1] = 42;
    ///     last_five[0] = 69;
    ///     assert_eq!(first_five[2], 2);
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_par_chunk_index(&mut self, chunk_size: usize) -> impl UnsafeChunkIndex<T> + ParView<[T]>;
}

/// A value-to-value conversion that consumes the input collection and produces one
/// that allows unsynchronized access to its elements.
///
/// This trait allows to convert a collection to allow unsynchronized access to its
/// elements, either one-by-one or in chunks of arbitrary size.
///
/// The different methods allow the user to choose the level of access to use:
/// * [`into_par_index_no_ref`](`Self::into_par_index_no_ref`) and
///   [`into_par_chunk_index_no_ref`](`Self::into_par_chunk_index_no_ref`) allow access
///   through [`UnsafeNoRefIndex`] and [`UnsafeNoRefChunkIndex`] respectively
///   (see module documentation for more information).
/// * [`into_pointer_par_index`](`Self::into_pointer_par_index`) and
///   [`into_pointer_par_chunk_index`](`Self::into_pointer_par_chunk_index`) allow access
///   through [`PointerIndex`] and [`PointerChunkIndex`] respectively
///   (see module documentation for more information).
/// * [`into_par_index`](`Self::into_par_index`) and
///   [`into_par_chunk_index`](`Self::into_par_chunk_index`) allow access
///   through [`UnsafeIndex`] and [`UnsafeChunkIndex`] respectively
///   (see module documentation for more information).
///
/// Unsafe code can rely on this trait behavior thanks to the invariants specified below.
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * [`into_par_index_no_ref`](`Self::into_par_index_no_ref`),
///   [`into_pointer_par_index`](`Self::into_pointer_par_index`) and
///   [`into_par_index`](`Self::into_par_index`) return collections
///   such that their [`len`](`TrustedSizedCollection::len`) is the size of the input
///   collection and that if index `i` refers to element `x` in the input collection, it refers
///   to element `x` in the returned collection as well.
/// * [`into_par_chunk_index_no_ref`](`Self::into_par_chunk_index_no_ref`),
///   [`into_pointer_par_chunk_index`](`Self::into_pointer_par_chunk_index`) and
///   [`into_par_chunk_index`](`Self::into_par_chunk_index`) panic if the
///   collection's size is not divisible by `chunk_size`.
/// * [`into_par_chunk_index_no_ref`](`Self::into_par_chunk_index_no_ref`),
///   [`into_pointer_par_chunk_index`](`Self::into_pointer_par_chunk_index`) and
///   [`into_par_chunk_index`](`Self::into_par_chunk_index`) return collections
///   such that their [`num_elements`](`TrustedChunkSizedCollection::num_elements`)
///   is equal to the size of the input collection,
///   [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`) is equal to the `chunk_size`
///   parameter passed to the method, [`len`](`TrustedSizedCollection::len`) is
///   equal to `num_elements / chunk_size` and chunk indices follow the input collection's
///   indices (*i.e.* chunk 0 of a collection of `chunk_size` 4 includes indices from 0 to 3,
///   chunk 1 includes indices from 4 to 7, etc.).
/// * All returned collections must implement [`Into`] to convert back to the original collection
///   type. The input collection's original internal state beside size is not guaranteed to be preserved
///   (*i.e.* a [`Vec`] can possess a different [`capacity`](Vec::capacity) when converted back, but
///   the [`len`](`Vec::len`) must be the same, as well as the indexes of its elements).
///
/// # Examples
///
/// We can convert back and forth between different paradigms:
/// ```
/// # use par_slice::*;
/// let mut collection = vec![0; 10];
///
/// // Let's use pointers to single elements
/// let par_collection = collection.into_pointer_par_index();
///
/// let ptr_1 = par_collection.get_mut_ptr(1);
/// unsafe {
///     *ptr_1 = 42;
/// }
///
/// collection = par_collection.into();
/// assert_eq!(collection, vec![0, 42, 0, 0, 0, 0, 0, 0, 0, 0]);
///
/// // Let's use setters and getters to chunks of size 2
/// let par_collection = collection.into_par_chunk_index_no_ref(2);
///
/// unsafe {
///     par_collection.set_values(1, &[69, 69]);
/// }
///
/// collection = par_collection.into();
/// assert_eq!(collection, vec![0, 42, 69, 69, 0, 0, 0, 0, 0, 0]);
///
/// // Let's use references to chunks of size 5
/// let par_collection = collection.into_par_chunk_index(5);
///
/// let last_five = unsafe { par_collection.get_mut(1) };
/// let mut i = 1;
/// for elem in last_five.iter_mut() {
///     *elem = i;
///     i += 1;
/// }
/// last_five[2] = 42;
///
/// collection = par_collection.into();
/// assert_eq!(collection, vec![0, 42, 69, 69, 0, 1, 2, 42, 4, 5]);
/// ```
pub unsafe trait IntoParIndex<T>: Sized {
    /// Converts the collection into one that allows unsynchronized access to
    /// its elements through pointers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_pointer_par_index();
    ///
    /// let mut_ptr_1 = collection.get_mut_ptr(1);
    /// let mut_ptr_5 = collection.get_mut_ptr(5);
    /// let ptr_2 = collection.get_ptr(2);
    /// unsafe {
    ///     *mut_ptr_1 = 42;
    ///     *mut_ptr_5 = 69;
    ///     assert_eq!(*ptr_2, 2);
    /// }
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_pointer_par_index(self) -> impl PointerIndex<T> + ParCollection<T, Self>;

    /// Converts the collection into one that allows unsynchronized access to its
    /// elements through setters and getters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_par_index_no_ref();
    ///
    /// unsafe {
    ///     collection.set_value(1, 42);
    ///     collection.set_value(5, 69);
    ///     assert_eq!(collection.get_value(2), 2);
    /// }
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_par_index_no_ref(self) -> impl UnsafeNoRefIndex<T> + ParCollection<T, Self>;

    /// Converts the collection into one that allows unsynchronized access to its
    /// elements through references.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_par_index();
    ///
    /// let mut_ref_1 = unsafe { collection.get_mut(1) };
    /// let mut_ref_5 = unsafe { collection.get_mut(5) };
    /// let ref_2 = unsafe { collection.get(2) };
    /// *mut_ref_1 = 42;
    /// *mut_ref_5 = 69;
    /// assert_eq!(*ref_2, 2);
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_par_index(self) -> impl UnsafeIndex<T> + ParCollection<T, Self>;

    /// Converts the collection into one that allows unsynchronized access to
    /// chunks of `chunk_size` of its elements through pointers.
    ///
    /// # Panics
    ///
    /// Panics if the size of the collection is not divisible by `chunk_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_pointer_par_chunk_index(5);
    ///
    /// let first_five = collection.get_mut_ptr(0);
    /// let last_five = collection.get_mut_ptr(1);
    /// unsafe {
    ///     (*first_five)[1] = 42;
    ///     (*last_five)[0] = 69;
    ///     assert_eq!((*first_five)[2], 2);
    /// }
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_pointer_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkIndex<T> + ParCollection<[T], Self>;

    /// Converts the collection into one that allows unsynchronized access to
    /// chunks of `chunk_size` of its elements through setters and getters.
    ///
    /// # Panics
    ///
    /// Panics if the size of the collection is not divisible by `chunk_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_par_chunk_index_no_ref(5);
    ///
    /// unsafe {
    ///     collection.set_values(0, &[0, 42, 2, 3, 4]);
    ///     collection.set_values(1, &[69, 6, 7, 8, 9]);
    ///     assert_eq!(collection.get_values(1, vec![0; 5]).as_ref(), vec![69, 6, 7, 8, 9]);
    /// }
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_par_chunk_index_no_ref(
        self,
        chunk_size: usize,
    ) -> impl UnsafeNoRefChunkIndex<T> + ParCollection<[T], Self>;

    /// Converts the collection into one that allows unsynchronized access to
    /// chunks of `chunk_size` of its elements through references.
    ///
    /// # Panics
    ///
    /// Panics if the size of the collection is not divisible by `chunk_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_par_chunk_index(5);
    ///
    /// let first_five = unsafe { collection.get_mut(0) };
    /// let last_five = unsafe { collection.get_mut(1) };
    /// first_five[1] = 42;
    /// last_five[0] = 69;
    /// assert_eq!(first_five[2], 2);
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_par_chunk_index(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkIndex<T> + ParCollection<[T], Self>;
}
