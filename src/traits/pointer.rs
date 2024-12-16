pub unsafe trait PointerParSlice<T: ?Sized> {
    fn len(&self) -> usize;

    fn get_ptr_unchecked(&self, index: usize) -> *const T;

    fn get_mut_ptr_unchecked(&self, index: usize) -> *mut T;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    fn get_ptr(&self, index: usize) -> *const T {
        assert!(
            index < self.len(),
            "Index {} invalid for slice of len {}",
            index,
            self.len()
        );
        self.get_ptr_unchecked(index)
    }

    #[inline(always)]
    fn get_mut_ptr(&self, index: usize) -> *mut T {
        assert!(
            index < self.len(),
            "Index {} invalid for slice of len {}",
            index,
            self.len()
        );
        self.get_mut_ptr_unchecked(index)
    }
}
