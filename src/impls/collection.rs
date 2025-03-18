use crate::*;

unsafe impl<T> TrustedSizedCollection<T> for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

unsafe impl<T> TrustedSizedCollection<T> for [T] {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

unsafe impl<T, const N: usize> TrustedSizedCollection<T> for [T; N] {
    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn is_empty(&self) -> bool {
        N == 0
    }
}

unsafe impl<T, const N: usize> TrustedSizedCollection<[T]> for [[T; N]] {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

unsafe impl<T, const N: usize> TrustedChunkSizedCollection<T> for [[T; N]] {
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

unsafe impl<T, const N: usize, const M: usize> TrustedSizedCollection<[T]> for [[T; N]; M] {
    #[inline]
    fn len(&self) -> usize {
        M
    }

    #[inline]
    fn is_empty(&self) -> bool {
        M == 0
    }
}

unsafe impl<T, const N: usize, const M: usize> TrustedChunkSizedCollection<T> for [[T; N]; M] {
    #[inline]
    fn chunk_size(&self) -> usize {
        N
    }

    #[inline]
    fn num_chunks(&self) -> usize {
        M
    }

    #[inline]
    fn num_elements(&self) -> usize {
        M * N
    }
}
