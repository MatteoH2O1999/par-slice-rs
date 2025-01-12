use crate::*;

unsafe impl<T> TrustedSizedCollection for Vec<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

unsafe impl<T> TrustedSizedCollection for [T] {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

unsafe impl<T, const N: usize> TrustedSizedCollection for [T; N] {
    #[inline(always)]
    fn len(&self) -> usize {
        N
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        N == 0
    }
}

unsafe impl<T, const N: usize> TrustedChunkSizedCollection for [[T; N]] {
    #[inline(always)]
    fn chunk_size(&self) -> usize {
        N
    }

    #[inline(always)]
    fn num_chunks(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn num_elements(&self) -> usize {
        self.len() * N
    }
}

unsafe impl<T, const N: usize, const M: usize> TrustedChunkSizedCollection for [[T; N]; M] {
    #[inline(always)]
    fn chunk_size(&self) -> usize {
        N
    }

    #[inline(always)]
    fn num_chunks(&self) -> usize {
        M
    }

    #[inline(always)]
    fn num_elements(&self) -> usize {
        M * N
    }
}
