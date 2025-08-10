/// Convert non indexing types to be used for unsafe idexing with
/// traits [`PointerIndex`](crate::PointerIndex), [`UnsafeNoRefIndex`](crate::UnsafeNoRefIndex)
/// and [`UnsafeIndex`](crate::UnsafeIndex) thanks to [`IndexWrapper`](crate::IndexWrapper).
///
/// # Safety
/// It must hold that `x != y <=> x.as_usize() != y.as_usize()`.
///
/// [`as_usize`](AsUsize::as_usize) may panic if `x` has no image in the codomain [`usize`].
pub unsafe trait AsUsize {
    /// Converts the input type into the indexing type [`usize`].
    ///
    /// # Panics
    ///
    /// Panics if `self` has no image of type [`usize`].
    fn as_usize(&self) -> usize;
}
