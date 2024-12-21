pub trait UnsafeAccess<T: ?Sized> {
    unsafe fn get(&self, index: usize) -> &T;

    unsafe fn get_unchecked(&self, index: usize) -> &T;

    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut(&self, index: usize) -> &mut T;

    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut_unchecked(&self, index: usize) -> &mut T;
}
