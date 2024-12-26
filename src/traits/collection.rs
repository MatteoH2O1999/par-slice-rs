pub unsafe trait TrustedSizedCollection {
    fn len(&self) -> usize;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub unsafe trait TrustedChunkSizedCollection: TrustedSizedCollection {
    fn chunk_size(&self) -> usize;

    #[inline(always)]
    fn num_elements(&self) -> usize {
        self.len() * self.chunk_size()
    }

    #[inline(always)]
    fn num_chunks(&self) -> usize {
        self.len()
    }
}

#[inline(always)]
pub(crate) fn assert_in_bounds(len: usize, index: usize) {
    assert!(
        index < len,
        "Index {} invalid for slice of len {}",
        index,
        len
    )
}

#[inline(always)]
pub(crate) fn assert_chunk_compatible<T>(chunk_size: usize, chunk: &[T]) {
    assert!(
        chunk.len() == chunk_size,
        "value should have the same length as the chunk. Got a value of length {} for a chunk of length {}",
        chunk.len(),
        chunk_size
    )
}
