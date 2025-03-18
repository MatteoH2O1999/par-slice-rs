use crate::*;

pub struct IndexWrapper<I, T, B> {
    backend: B,
    _marker: std::marker::PhantomData<(I, T)>,
}

impl<T, B: TrustedSizedCollection<T>> IndexWrapper<(), T, B> {
    #[inline]
    pub fn new<Idx: AsUsize>(backend: B) -> IndexWrapper<Idx, T, B> {
        IndexWrapper {
            backend,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<I, T, B> IndexWrapper<I, T, B> {
    #[inline]
    pub fn into_inner(self) -> B {
        self.backend
    }
}
