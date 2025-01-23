use crate::*;
use std::fmt::Debug;

pub struct PointerParSlice;

impl PointerParSlice {
    #[allow(clippy::new_ret_no_self)]
    #[inline(always)]
    pub fn new<T: Default + Send>(
        len: usize,
    ) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice(len).into_pointer_par_slice()
    }

    #[inline(always)]
    pub fn with_value<T: Clone + Send>(
        value: T,
        len: usize,
    ) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice_with_value(len, value).into_pointer_par_slice()
    }

    #[inline(always)]
    pub fn with_closure<T: Send>(
        closure: impl FnMut(usize) -> T,
        len: usize,
    ) -> impl PointerAccess<T> + Into<Box<[T]>> + Sync + Debug {
        new_boxed_slice_with(len, closure).into_pointer_par_slice()
    }

    #[inline(always)]
    pub fn new_chunks<T: Default + Send>(
        len: usize,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice(len).into_pointer_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    pub fn chunks_with_value<T: Clone + Send>(
        value: T,
        len: usize,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with_value(len, value).into_pointer_par_chunk_slice(chunk_size)
    }

    #[inline(always)]
    pub fn chunks_with_closure<T: Send>(
        closure: impl FnMut(usize) -> T,
        len: usize,
        chunk_size: usize,
    ) -> impl PointerChunkAccess<T> + Into<Box<[T]>> + Sync + Debug {
        assert_chunk_size(len, chunk_size);
        new_boxed_slice_with(len, closure).into_pointer_par_chunk_slice(chunk_size)
    }
}
