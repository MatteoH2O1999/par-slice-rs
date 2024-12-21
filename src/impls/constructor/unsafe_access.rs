use crate::{constructor::*, *};

pub struct UnsafeParSlice;

impl UnsafeParSlice {
    #[inline(always)]
    pub fn new<T: Default + Sync>(len: usize) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync {
        new_boxed_slice(len).into_unsafe_par_slice()
    }

    #[inline(always)]
    pub fn with_value<T: Clone + Sync>(
        value: T,
        len: usize,
    ) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync {
        new_boxed_slice_with_value(len, value).into_unsafe_par_slice()
    }

    #[inline(always)]
    pub fn with_closure<T: Sync>(
        closure: impl FnMut() -> T,
        len: usize,
    ) -> impl UnsafeAccess<T> + Into<Box<[T]>> + Sync {
        new_boxed_slice_with(len, closure).into_unsafe_par_slice()
    }

    #[inline(always)]
    pub fn new_chunks<T: Default + Sync>(
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeAccess<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice(len).into_unsafe_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    pub fn chunks_with_value<T: Clone + Sync>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeAccess<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with_value(len, value).into_unsafe_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    pub fn chunks_with_closure<T: Sync>(
        closure: impl FnMut() -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl UnsafeAccess<[T]> + Into<Box<[T]>> + Sync {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with(len, closure).into_unsafe_par_chunk_slice(chunk_size)
    }
}
