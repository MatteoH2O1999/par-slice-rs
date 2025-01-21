use crate::*;
use std::fmt::Debug;

/// View of a collection that allows unsynchronized access to its elements.
///
/// This trait allows to temporarily opt-in to unsynchronized access to the elements of
/// the underlying collection, either one-by-one or in chunks of arbitrary size.
///
/// The different methods allow the user to choose the level of access to use:
/// * [`as_data_race_par_slice`](`Self::as_data_race_par_slice`) and
///   [`as_data_race_par_chunk_slice`](`Self::as_data_race_par_chunk_slice`) allow access
///   through [`UnsafeDataRaceAccess`] and [`UnsafeDataRaceChunkAccess`] respectively
///   (see module documentation for more information).
/// * [`as_pointer_par_slice`](`Self::as_pointer_par_slice`) and
///   [`as_pointer_par_chunk_slice`](`Self::as_pointer_par_chunk_slice`) allow access
///   through [`PointerAccess`] and [`PointerChunkAccess`] respectively
///   (see module documentation for more information).
/// * [`as_unsafe_par_slice`](`Self::as_unsafe_par_slice`) and
///   [`as_unsafe_par_chunk_slice`](`Self::as_unsafe_par_chunk_slice`) allow access
///   through [`UnsafeAccess`] and [`UnsafeChunkAccess`] respectively
///   (see module documentation for more information).
///
/// Unsafe code can rely on this trait behavior thanks to the invariants specified below.
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * [`as_data_race_par_slice`](`Self::as_data_race_par_slice`),
///   [`as_pointer_par_slice`](`Self::as_pointer_par_slice`) and
///   [`as_unsafe_par_slice`](`Self::as_unsafe_par_slice`) return views on the collection
///   such that their [`len`](`TrustedSizedCollection::len`) is the size of the
///   collection and that if index `i` refers to element `x` in the collection, it refers
///   to element `x` in the returned views as well.
/// * [`as_data_race_par_chunk_slice`](`Self::as_data_race_par_chunk_slice`),
///   [`as_pointer_par_chunk_slice`](`Self::as_pointer_par_chunk_slice`) and
///   [`as_unsafe_par_chunk_slice`](`Self::as_unsafe_par_chunk_slice`) panic if the
///   collection's size is not divisible by `chunk_size`.
/// * [`as_data_race_par_chunk_slice`](`Self::as_data_race_par_chunk_slice`),
///   [`as_pointer_par_chunk_slice`](`Self::as_pointer_par_chunk_slice`) and
///   [`as_unsafe_par_chunk_slice`](`Self::as_unsafe_par_chunk_slice`) return views on the
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
///     let view = collection.as_pointer_par_slice();
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
///     let view = collection.as_data_race_par_chunk_slice(2);
///     unsafe {
///         view.set(1, &[69, 69]);
///     }
/// }
///
/// assert_eq!(collection, vec![0, 42, 69, 69, 0, 0, 0, 0, 0, 0]);
///
/// {
///     // Let's use references to chunks of size 5
///     let view = collection.as_unsafe_par_chunk_slice(5);
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
pub unsafe trait ParSliceView<T> {
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
    ///     let view = collection.as_pointer_par_slice();
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
    fn as_pointer_par_slice(&mut self) -> impl PointerAccess<T> + Sync + Debug;

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
    ///     let view = collection.as_data_race_par_slice();
    ///     unsafe {
    ///         view.set(1, 42);
    ///         view.set(5, 69);
    ///         assert_eq!(view.get(2), 2);
    ///     }
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_data_race_par_slice(&mut self) -> impl UnsafeDataRaceAccess<T> + Sync + Debug;

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
    ///     let view = collection.as_unsafe_par_slice();
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
    fn as_unsafe_par_slice(&mut self) -> impl UnsafeAccess<T> + Sync + Debug;

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
    ///     let view = collection.as_pointer_par_chunk_slice(5);
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
    fn as_pointer_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Sync + Debug;

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
    ///     let view = collection.as_data_race_par_chunk_slice(5);
    ///     unsafe {
    ///         view.set(0, &[0, 42, 2, 3, 4]);
    ///         view.set(1, &[69, 6, 7, 8, 9]);
    ///         assert_eq!(view.get(1).as_ref(), [69, 6, 7, 8, 9]);
    ///     }
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_data_race_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Sync + Debug;

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
    ///     let view = collection.as_unsafe_par_chunk_slice(5);
    ///     let first_five = unsafe { view.get_mut(0) };
    ///     let last_five = unsafe { view.get_mut(1) };
    ///     first_five[1] = 42;
    ///     last_five[0] = 69;
    ///     assert_eq!(first_five[2], 2);
    /// }
    ///
    /// assert_eq!(collection, vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn as_unsafe_par_chunk_slice(
        &mut self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Sync + Debug;
}

/// A value-to-value conversion that consumes the input collection and produces one
/// that allows unsynchronized access to its elements.
///
/// This trait allows to convert a collection to allow unsynchronized access to its
/// elements, either one-by-one or in chunks of arbitrary size.
///
/// The different methods allow the user to choose the level of access to use:
/// * [`into_data_race_par_slice`](`Self::into_data_race_par_slice`) and
///   [`into_data_race_par_chunk_slice`](`Self::into_data_race_par_chunk_slice`) allow access
///   through [`UnsafeDataRaceAccess`] and [`UnsafeDataRaceChunkAccess`] respectively
///   (see module documentation for more information).
/// * [`into_pointer_par_slice`](`Self::into_pointer_par_slice`) and
///   [`into_pointer_par_chunk_slice`](`Self::into_pointer_par_chunk_slice`) allow access
///   through [`PointerAccess`] and [`PointerChunkAccess`] respectively
///   (see module documentation for more information).
/// * [`into_unsafe_par_slice`](`Self::into_unsafe_par_slice`) and
///   [`into_unsafe_par_chunk_slice`](`Self::into_unsafe_par_chunk_slice`) allow access
///   through [`UnsafeAccess`] and [`UnsafeChunkAccess`] respectively
///   (see module documentation for more information).
///
/// Unsafe code can rely on this trait behavior thanks to the invariants specified below.
///
/// # Safety
///
/// Implementors of this trait must guarantee the following invariants:
/// * [`into_data_race_par_slice`](`Self::into_data_race_par_slice`),
///   [`into_pointer_par_slice`](`Self::into_pointer_par_slice`) and
///   [`into_unsafe_par_slice`](`Self::into_unsafe_par_slice`) return collections
///   such that their [`len`](`TrustedSizedCollection::len`) is the size of the input
///   collection and that if index `i` refers to element `x` in the input collection, it refers
///   to element `x` in the returned collection as well.
/// * [`into_data_race_par_chunk_slice`](`Self::into_data_race_par_chunk_slice`),
///   [`into_pointer_par_chunk_slice`](`Self::into_pointer_par_chunk_slice`) and
///   [`into_unsafe_par_chunk_slice`](`Self::into_unsafe_par_chunk_slice`) panic if the
///   collection's size is not divisible by `chunk_size`.
/// * [`into_data_race_par_chunk_slice`](`Self::into_data_race_par_chunk_slice`),
///   [`into_pointer_par_chunk_slice`](`Self::into_pointer_par_chunk_slice`) and
///   [`into_unsafe_par_chunk_slice`](`Self::into_unsafe_par_chunk_slice`) return collections
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
/// let par_collection = collection.into_pointer_par_slice();
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
/// let par_collection = collection.into_data_race_par_chunk_slice(2);
///
/// unsafe {
///     par_collection.set(1, &[69, 69]);
/// }
///
/// collection = par_collection.into();
/// assert_eq!(collection, vec![0, 42, 69, 69, 0, 0, 0, 0, 0, 0]);
///
/// // Let's use references to chunks of size 5
/// let par_collection = collection.into_unsafe_par_chunk_slice(5);
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
pub unsafe trait IntoParSlice<T>: Sized {
    /// Converts the collection into one that allows unsynchronized access to
    /// its elements through pointers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_pointer_par_slice();
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
    fn into_pointer_par_slice(self) -> impl PointerAccess<T> + Into<Self> + Sync + Debug;

    /// Converts the collection into one that allows unsynchronized access to its
    /// elements through setters and getters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_data_race_par_slice();
    ///
    /// unsafe {
    ///     collection.set(1, 42);
    ///     collection.set(5, 69);
    ///     assert_eq!(collection.get(2), 2);
    /// }
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_data_race_par_slice(self) -> impl UnsafeDataRaceAccess<T> + Into<Self> + Sync + Debug;

    /// Converts the collection into one that allows unsynchronized access to its
    /// elements through references.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_unsafe_par_slice();
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
    fn into_unsafe_par_slice(self) -> impl UnsafeAccess<T> + Into<Self> + Sync + Debug;

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
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_pointer_par_chunk_slice(5);
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
    fn into_pointer_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Into<Self> + Sync + Debug;

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
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_data_race_par_chunk_slice(5);
    ///
    /// unsafe {
    ///     collection.set(0, &[0, 42, 2, 3, 4]);
    ///     collection.set(1, &[69, 6, 7, 8, 9]);
    ///     assert_eq!(collection.get(1).as_ref(), [69, 6, 7, 8, 9]);
    /// }
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_data_race_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeDataRaceChunkAccess<T> + Into<Self> + Sync + Debug;

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
    /// let collection = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_unsafe_par_chunk_slice(5);
    ///
    /// let first_five = unsafe { collection.get_mut(0) };
    /// let last_five = unsafe { collection.get_mut(1) };
    /// first_five[1] = 42;
    /// last_five[0] = 69;
    /// assert_eq!(first_five[2], 2);
    ///
    /// assert_eq!(collection.into(), vec![0, 42, 2, 3, 4, 69, 6, 7, 8, 9]);
    /// ```
    fn into_unsafe_par_chunk_slice(
        self,
        chunk_size: usize,
    ) -> impl UnsafeChunkAccess<T> + Into<Self> + Sync + Debug;
}
