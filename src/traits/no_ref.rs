use crate::*;

/// Unsynchronized access to elements of a collection through setters and getters without
/// crating references to its elements.
///
/// The trait allows *unsynchronized* access to the elements of a collection by
/// allowing the use of setters and getters from a *shared reference* to the
/// collection and its index.
///
/// The user is responsible to avoid data races.
///
/// For more details see the individual methods.
///
/// # Safety
///
/// Implementors must be careful of [undefined behavior] when mutating an element from a shared reference.
/// It is advisable to learn about [interior mutability](https://doc.rust-lang.org/reference/interior-mutability.html)
/// before trying to implement this trait.
///
/// Implementors of this trait must guarantee the following invariants:
/// * The collection has size [`len`](`TrustedSizedCollection::len`).
/// * For each collection of size `n`, indexes are defined from `0` to `n - 1`, each univocally identifying an element in
///   the collection.
/// * For each index `i`, `collection.get_value(i)` returns a bitwise copy of the element identified by index `i` in the collection,
///   panicking whenever `i` is out of bounds. It is still up to the caller to ensure no data races can happen during the read.
/// * For each index `i`, `collection.get_value_unchecked(i)` returns a bitwise copy of the element identified by index `i` in the collection.
///   It is up to the caller to ensure no data races can happen during the read.
/// * For each index `i`, `collection.set_value(i, value)` sets the element identified by index `i` in the collection to `value`,
///   panicking whenever `i` is out of bounds. It is still up to the caller to ensure no data races can happen during the write.
/// * For each index `i`, `collection.set_value_unchecked(i, value)` sets the element identified by index `i` in the collection to `value`.
///   It is up to the caller to ensure no data races can happen during the write.
/// * For each valid index `i`, `collection.get_value(i) == collection.get_value_unchecked(i)`.
/// * No references to elements in the collection are ever created.
///
/// # Examples
///
/// There are no ways of obtaining undefined behavior using checked methods in single threaded code:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_par_index_no_ref();
///
/// unsafe {
///     // This is single threaded so no data races can happen
///     collection.set_value(0, 42);
///     collection.set_value(1, 42);
///     assert_eq!(collection.get_value(0), 42);
///     collection.set_value(0, 69);
/// }
///
/// assert_eq!(collection.into().as_ref(), vec![69, 42, 0, 0, 0]);
/// ```
///
/// There are no ways of obtaining undefined behavior using unchecked methods in single threaded code with indexes
/// that are sure to be in bounds:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 5].into_par_index_no_ref();
///
/// unsafe {
///     // This is single threaded so no data races can happen and 0 and 1 are valid indexes
///     // for a collection of length 5
///     collection.set_value_unchecked(0, 42);
///     collection.set_value_unchecked(1, 42);
///     assert_eq!(collection.get_value_unchecked(0), 42);
///     collection.set_value_unchecked(0, 69);
/// }
///
/// assert_eq!(collection.into().as_ref(), vec![69, 42, 0, 0, 0]);
/// ```
///
/// When in a parallel context indexes must be unique in order to avoid data races:
///
/// ```
/// # use par_slice::*;
/// # use std::thread::scope;
/// let collection = vec![0; 5].into_par_index_no_ref();
///
/// scope(|s|{
///     s.spawn(||{
///         for i in [0, 1] {
///             unsafe {
///                 // 0 and 1 are valid indexes and next thread
///                 // does not access them
///                 collection.set_value_unchecked(i, 42);
///             }
///         }
///     });
///     s.spawn(||{
///         for i in [3, 4] {
///             unsafe {
///                 // 3 and 4 are valid indexes and previous thread
///                 // does not access them
///                 collection.set_value_unchecked(i, 69);
///             }
///         }
///     });
/// });
///
/// assert_eq!(collection.into().as_ref(), vec![42, 42, 0, 69, 69]);
/// ```
///
/// A possible data race is undefined behavior:
///
/// ```no_run
/// # use par_slice::*;
/// # use std::thread::scope;
/// let collection = vec![0; 5].into_par_index_no_ref();
///
/// scope(|s|{
///     s.spawn(||{
///         for i in 0..5 {
///             unsafe {
///                 // 0, 1, 2, 3 and 4 are valid indexes
///                 collection.set_value_unchecked(i, 42);
///             }
///         }
///     });
///     s.spawn(||{
///         for i in 0..5 {
///             unsafe {
///                 // This thread accesses the same indexes as the previous
///                 // one leading to a possible data race: this is UB
///                 collection.set_value_unchecked(i, 69);
///             }
///         }
///     });
/// });
/// ```
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub unsafe trait UnsafeNoRefIndex<T: ?Sized>: TrustedSizedCollection<T> {
    /// Returns a bitwise copy of the element identified by `index` in the collection.
    ///
    /// This method performs bounds checking on `index` to ensure its validity.
    /// If you can guarantee its validity, you may want to use the [`get_value_unchecked`](`Self::get_value_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing to the same element from another thread is undefined behavior
    /// (parallel reads are ok).
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index_no_ref();
    /// // This is single threaded so no data races can happen
    /// assert_eq!(unsafe { collection.get_value(0) }, 0);
    /// ```
    #[inline]
    unsafe fn get_value(&self, index: usize) -> T
    where
        T: Copy,
    {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_value_unchecked(index)
        }
    }

    /// Returns a bitwise copy of the element identified by `index` in the collection, without performing
    /// bounds checking.
    ///
    /// This method does not perform bounds checking on `index` to ensure its validity.
    /// If you can't guarantee its validity, you may want to use the [`get_value`](`Self::get_value`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing to the same element from another thread is undefined behavior
    /// (parallel reads are ok).
    /// Calling this method with an index `i` that would panic [`get_value`](`Self::get_value`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index_no_ref();
    /// // We know 0 is a valid index for a collection of length 5
    /// // and this is single threaded so no data races can happen
    /// assert_eq!(unsafe { collection.get_value_unchecked(0) }, 0);
    /// ```
    unsafe fn get_value_unchecked(&self, index: usize) -> T
    where
        T: Copy;

    /// Sets the element identified by `index` in the collection to `value`.
    ///
    /// This method performs bounds checking on `index` to ensure its validity.
    /// If you can guarantee its validity, you may want to use the [`set_value_unchecked`](`Self::set_value_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing or reading the same element from another thread
    /// is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index_no_ref();
    ///
    /// // This is single threaded so no data races can happen
    /// unsafe { collection.set_value(0, 42) };
    ///
    /// assert_eq!(collection.into(), vec![42, 0, 0, 0, 0]);
    /// ```
    #[inline]
    unsafe fn set_value(&self, index: usize, value: T)
    where
        T: Sized,
    {
        assert_in_bounds(self.len(), index);
        unsafe {
            // Safety: we just checked that index is in bounds
            self.set_value_unchecked(index, value);
        }
    }

    /// Sets the element identified by `index` in the collection to `value`, without performing
    /// bounds checking.
    ///
    /// This method does not perform bounds checking on `index` to ensure its validity.
    /// If you can't guarantee its validity, you may want to use the [`set_value`](`Self::set_value`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing or reading the same element from another thread
    /// is undefined behavior.
    /// Calling this method with an index `i` that would panic [`set_value`](`Self::set_value`) is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 5].into_par_index_no_ref();
    ///
    /// // We know 0 is a valid index for a collection of length 5
    /// // and this is single threaded so no data races can happen
    /// unsafe { collection.set_value_unchecked(0, 42) };
    ///
    /// assert_eq!(collection.into(), vec![42, 0, 0, 0, 0]);
    /// ```
    unsafe fn set_value_unchecked(&self, index: usize, value: T)
    where
        T: Sized;
}

/// Unsynchronized access to chunks of elements of a collection through setters and getters without
/// crating references to its elements.
///
/// The trait allows *unsynchronized* access to chunks of elements elements of a collection by
/// allowing the use of setters and getters from a *shared reference* to the
/// collection and its index.
///
/// The user is responsible to avoid data races.
///
/// For more details see the individual methods.
///
/// # Safety
///
/// Implementors must be careful of [undefined behavior] when mutating an element from a shared reference.
/// It is advisable to learn about [interior mutability](https://doc.rust-lang.org/reference/interior-mutability.html)
/// before trying to implement this trait.
///
/// Implementors of this trait must guarantee the following invariants:
/// * The collection contains [`num_chunks`](`TrustedChunkSizedCollection::num_chunks`) **non-overlapping** slices of `T`,
///   each of len [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`).
/// * For each collection of size `n`, chunk indexes are defined from `0` to `n - 1`, each univocally identifying a chunk of elements in
///   the collection as follows: a chunk of index `i` includes all elements from index `i * collection.chunk_size()` included to
///   `(i + 1) * collection.chunk_size()` excluded.
/// * For each index `i`, `collection.get_values(i, out)` sets `out` to a bitwise copy of the chunk of elements identified
///   by index `i` in the collection, panicking whenever `i` is out of bounds or `out` has not the same size as
///   [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`). It is still up to the caller to ensure no data races can happen during the read.
/// * For each index `i`, `collection.get_values_unchecked(i, out)` sets `out` to a bitwise copy of the chunk of elements
///   identified by index `i` in the collection. It is up to the caller to ensure no data races can happen during the read and that `out`
///   is a slice of the correct length.
/// * For each index `i`, `collection.set_values(i, values)` sets the chunk of elements identified by index `i` in the collection to `values`,
///   panicking whenever `i` is out of bounds or `values` has not the same size as [`chunk_size`](`TrustedChunkSizedCollection::chunk_size`).
///   It is still up to the caller to ensure no data races can happen during the write.
/// * For each index `i`, `collection.set_values_unchecked(i, values)` sets the chunk of elements identified by index `i` in the collection to `values`.
///   It is up to the caller to ensure no data races can happen during the write and that `values` is a slice of the correct length.
/// * For each valid index `i`, `collection.get_values(i) == collection.get_values_unchecked(i)`.
/// * No references to elements in the collection are ever created.
///
/// # Examples
///
/// There are no ways of obtaining undefined behavior using checked methods in single threaded code:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 6].into_par_chunk_index_no_ref(2);
///
/// unsafe {
///     // This is single threaded so no data races can happen
///     collection.set_values(0, &[42, 69]);
///     collection.set_values(1, &[42, 69]);
///     assert_eq!(collection.get_values(0, vec![0; 2]), vec![42, 69]);
///     collection.set_values(0, &[69, 42]);
/// }
///
/// assert_eq!(collection.into().as_ref(), vec![69, 42, 42, 69, 0, 0]);
/// ```
///
/// There are no ways of obtaining undefined behavior using unchecked methods in single threaded code with indexes
/// that are sure to be in bounds:
///
/// ```
/// # use par_slice::*;
/// let collection = vec![0; 6].into_par_chunk_index_no_ref(2);
///
/// unsafe {
///     // This is single threaded so no data races can happen and 0 and 1 are valid indexes
///     // for a collection of length 6 with chunks of size 2
///     collection.set_values_unchecked(0, &[42, 69]);
///     collection.set_values_unchecked(1, &[42, 69]);
///     assert_eq!(collection.get_values_unchecked(0, vec![0; 2]), vec![42, 69]);
///     collection.set_values_unchecked(0, &[69, 42]);
/// }
///
/// assert_eq!(collection.into().as_ref(), vec![69, 42, 42, 69, 0, 0]);
/// ```
///
/// When in a parallel context indexes must be unique in order to avoid data races:
///
/// ```
/// # use par_slice::*;
/// # use std::thread::scope;
/// let collection = vec![0; 10].into_par_chunk_index_no_ref(2);
///
/// scope(|s|{
///     s.spawn(||{
///         for i in [0, 1] {
///             unsafe {
///                 // 0 and 1 are valid indexes and next thread
///                 // does not access them
///                 collection.set_values_unchecked(i, &[42, 69]);
///             }
///         }
///     });
///     s.spawn(||{
///         for i in [3, 4] {
///             unsafe {
///                 // 3 and 4 are valid indexes and previous thread
///                 // does not access them
///                 collection.set_values_unchecked(i, &[69, 42]);
///             }
///         }
///     });
/// });
///
/// assert_eq!(collection.into().as_ref(), vec![42, 69, 42, 69, 0, 0, 69, 42, 69, 42]);
/// ```
///
/// A possible data race is undefined behavior:
///
/// ```no_run
/// # use par_slice::*;
/// # use std::thread::scope;
/// let collection = vec![0; 10].into_par_chunk_index_no_ref(2);
///
/// scope(|s|{
///     s.spawn(||{
///         for i in 0..5 {
///             unsafe {
///                 // 0, 1, 2, 3 and 4 are valid indexes
///                 collection.set_values_unchecked(i, &[42, 69]);
///             }
///         }
///     });
///     s.spawn(||{
///         for i in 0..5 {
///             unsafe {
///                 // This thread accesses the same indexes as the previous
///                 // one leading to a possible data race: this is UB
///                 collection.set_values_unchecked(i, &[69, 42]);
///             }
///         }
///     });
/// });
/// ```
///
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub unsafe trait UnsafeNoRefChunkIndex<T>: TrustedChunkSizedCollection<T> {
    /// Sets `out` to a bitwise copy of the chunk of elements identified by `index` in
    /// the collection and returns `out`.
    ///
    /// This method performs runtime checks on `index` and `out` to ensure their validity.
    /// If you can guarantee their validity, you may want to use the [`get_values_unchecked`](`Self::get_values_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection or if `out.len() != self.chunk_size()`.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing to the same chunk from another thread is undefined behavior
    /// (parallel reads are ok).
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 10].into_par_chunk_index_no_ref(2);
    /// let mut buf = vec![0; 2];
    ///
    /// // This is single threaded so no data races can happen
    /// unsafe {
    ///     collection.get_values(0, &mut buf);
    /// }
    ///
    /// assert_eq!(buf, vec![0, 0]);
    /// ```
    #[inline]
    unsafe fn get_values<O: AsMut<[T]>>(&self, index: usize, mut out: O) -> O
    where
        T: Copy,
    {
        assert_in_bounds(self.len(), index);
        assert_chunk_compatible(self.chunk_size(), out.as_mut());
        unsafe {
            // Safety: we just checked that index is in bounds
            self.get_values_unchecked(index, out)
        }
    }

    /// Sets `out` to a bitwise copy of the chunk of elements identified by `index` in the collection and returns `out`,
    /// without performing bounds checking.
    ///
    /// This method does not perform runtime checks on `index` or `out` to ensure their validity.
    /// If you can't guarantee their validity, you may want to use the [`get_values`](`Self::get_values`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing to the same chunk from another thread is undefined behavior
    /// (parallel reads are ok).
    /// Calling this method with an index `i` or an output `out` that would panic [`get_values`](`Self::get_values`)
    /// is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 10].into_par_chunk_index_no_ref(2);
    /// let mut buf = vec![0; 2];
    ///
    /// // We know 0 is a valid index for a collection of length 5
    /// // and this is single threaded so no data races can happen
    /// unsafe {
    ///     collection.get_values_unchecked(0, &mut buf);
    /// }
    ///
    /// assert_eq!(buf, vec![0, 0]);
    /// ```
    unsafe fn get_values_unchecked<O: AsMut<[T]>>(&self, index: usize, out: O) -> O
    where
        T: Copy;

    /// Sets the chunk of elements identified by `index` in the collection to `values`.
    ///
    /// This method performs runtime checks on `index` and `values` to ensure their validity.
    /// If you can guarantee their validity, you may want to use the [`set_values_unchecked`](`Self::set_values_unchecked`)
    /// method instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds of the collection or if `values.len() != self.chunk_size()`.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing or reading the same chunk from another thread
    /// is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 10].into_par_chunk_index_no_ref(2);
    ///
    /// // This is single threaded so no data races can happen
    /// unsafe { collection.set_values(0, &[42, 69]) };
    ///
    /// assert_eq!(collection.into(), vec![42, 69, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    #[inline]
    unsafe fn set_values(&self, index: usize, values: &[T])
    where
        T: Clone,
    {
        assert_in_bounds(self.len(), index);
        assert_chunk_compatible(self.chunk_size(), values);
        unsafe {
            // Safety: we just checked that index is in bounds and value is compatible
            // with chunk_size
            self.set_values_unchecked(index, values);
        }
    }

    /// Sets the chunk of elements identified by `index` in the collection to `values`, without performing
    /// runtime checks on the arguments.
    ///
    /// This method does not perform runtime checks on `index` and `values` to ensure their validity.
    /// If you can't guarantee their validity, you may want to use the [`set_values`](`Self::set_values`) method instead.
    ///
    /// # Safety
    ///
    /// Calling this method while also writing or reading the same chunk from another thread
    /// is undefined behavior.
    /// Calling this method with an index `i` or a values `v` that would panic [`set_values`](`Self::set_values`)
    /// is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use par_slice::*;
    /// let collection = vec![0; 10].into_par_chunk_index_no_ref(2);
    ///
    /// // We know 0 is a valid index for a collection of length 5
    /// // and this is single threaded so no data races can happen
    /// unsafe { collection.set_values_unchecked(0, &[42, 69]) };
    ///
    /// assert_eq!(collection.into(), vec![42, 69, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    unsafe fn set_values_unchecked(&self, index: usize, values: &[T])
    where
        T: Clone;
}
