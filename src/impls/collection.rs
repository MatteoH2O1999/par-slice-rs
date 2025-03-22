use crate::*;

unsafe impl<T> TrustedSizedCollection for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

unsafe impl<T> TrustedSizedCollection for [T] {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

unsafe impl<T, const N: usize> TrustedSizedCollection for [T; N] {
    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn is_empty(&self) -> bool {
        N == 0
    }
}

unsafe impl<T, const N: usize> TrustedChunkSizedCollection for [[T; N]] {
    #[inline]
    fn chunk_size(&self) -> usize {
        N
    }

    #[inline]
    fn num_chunks(&self) -> usize {
        self.len()
    }

    #[inline]
    fn num_elements(&self) -> usize {
        self.len() * N
    }
}

unsafe impl<T, const N: usize, const M: usize> TrustedChunkSizedCollection for [[T; N]; M] {
    #[inline]
    fn chunk_size(&self) -> usize {
        N
    }

    #[inline]
    fn num_chunks(&self) -> usize {
        self.len()
    }

    #[inline]
    fn num_elements(&self) -> usize {
        M * N
    }
}
