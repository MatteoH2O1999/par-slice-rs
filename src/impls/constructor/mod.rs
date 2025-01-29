mod no_ref;
pub use no_ref::*;

mod pointer;
pub use pointer::*;

mod unsafe_index;
pub use unsafe_index::*;

/// Creates a new boxed slice of `len` elements, each initialized to the return value
/// of `closure`.
pub(crate) fn new_boxed_slice_with<T>(len: usize, mut closure: impl FnMut(usize) -> T) -> Box<[T]> {
    let mut boxed = Box::new_uninit_slice(len);
    for (i, elem) in boxed.iter_mut().enumerate() {
        elem.write(closure(i));
    }
    unsafe { boxed.assume_init() }
}

/// Creates a new boxed slice of `len` elements, each initialized to `value`.
pub(crate) fn new_boxed_slice_with_value<T: Clone>(len: usize, value: T) -> Box<[T]> {
    let mut boxed = Box::new_uninit_slice(len);
    if let Some((first, elems)) = boxed.split_first_mut() {
        for elem in elems {
            elem.write(value.clone());
        }
        first.write(value);
    }
    unsafe { boxed.assume_init() }
}

/// Creates a new boxed slice of `len` elements, each initialized to
/// [`T::default`](`Default::default`).
#[inline]
pub(crate) fn new_boxed_slice<T: Default>(len: usize) -> Box<[T]> {
    new_boxed_slice_with(len, |_| T::default())
}
