pub struct IndexWrapper<I, T, B> {
    backend: B,
    _marker: std::marker::PhantomData<(I, T)>,
}

impl<I, T, B> IndexWrapper<I, T, B> {
    #[inline]
    pub fn into_inner(self) -> B {
        self.backend
    }
}
