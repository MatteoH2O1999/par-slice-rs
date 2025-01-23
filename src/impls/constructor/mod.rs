mod data_race;
pub use data_race::*;

mod pointer;
pub use pointer::*;

mod unsafe_access;
pub use unsafe_access::*;

/// Creates a new boxed slice of `len` elements, each initialized with the return value
/// of `closure`.
#[inline(always)]
fn new_boxed_slice_with<T>(len: usize, mut closure: impl FnMut() -> T) -> Box<[T]> {
    let mut boxed = Box::new_uninit_slice(len);
    for elem in boxed.iter_mut() {
        elem.write(closure());
    }
    unsafe { boxed.assume_init() }
}

/// Creates a new boxed slice of `len` elements, each initialized with `value`.
#[inline(always)]
fn new_boxed_slice_with_value<T: Clone>(len: usize, value: T) -> Box<[T]> {
    let mut boxed = Box::new_uninit_slice(len);
    if let Some((first, elems)) = boxed.split_first_mut() {
        for elem in elems {
            elem.write(value.clone());
        }
        first.write(value);
    }
    unsafe { boxed.assume_init() }
}

/// Creates a new boxed slice of `len` elements, each initialized with
/// [`T::default`](`Default::default`).
#[inline(always)]
fn new_boxed_slice<T: Default>(len: usize) -> Box<[T]> {
    new_boxed_slice_with(len, T::default)
}
