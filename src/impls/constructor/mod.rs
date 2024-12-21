mod data_race;
pub use data_race::*;

mod pointer;
pub use pointer::*;

mod unsafe_access;
pub use unsafe_access::*;

#[inline(always)]
fn new_boxed_slice_with<T: Sync>(len: usize, mut closure: impl FnMut() -> T) -> Box<[T]> {
    let mut boxed = Box::new_uninit_slice(len);
    for elem in boxed.iter_mut() {
        elem.write(closure());
    }
    unsafe { boxed.assume_init() }
}

#[inline(always)]
fn new_boxed_slice_with_value<T: Sync + Clone>(len: usize, value: T) -> Box<[T]> {
    new_boxed_slice_with(len, || value.clone())
}

#[inline(always)]
fn new_boxed_slice<T: Sync + Default>(len: usize) -> Box<[T]> {
    new_boxed_slice_with(len, T::default)
}
