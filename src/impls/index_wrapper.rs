use crate::*;

pub struct IndexWrapper<I, T: ?Sized, B> {
    backend: B,
    _marker: std::marker::PhantomData<(I, T)>,
}

impl<T: ?Sized, B: ParView<T>> IndexWrapper<(), T, B> {
    pub fn new<I: AsUsize>(collection: B) -> IndexWrapper<I, T, B> {
        IndexWrapper {
            backend: collection,
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

impl<I: AsUsize, T: ?Sized, B: PointerIndex<T>> IndexWrapper<I, T, B> {
    /// Semantically equivalent to
    /// [`PointerIndex::get_ptr`](`PointerIndex::get_ptr`)`(i)` where `i` is
    /// `index.`[`as_usize`](`AsUsize::as_usize`).
    #[inline]
    pub fn get_ptr(&self, index: I) -> *const T {
        self.backend.get_ptr(index.as_usize())
    }

    /// Semantically equivalent to
    /// [`PointerIndex::get_ptr_unchecked`](`PointerIndex::get_ptr_unchecked`)`(i)` where `i` is
    /// `index.`[`as_usize`](`AsUsize::as_usize`).
    #[inline]
    pub unsafe fn get_ptr_unchecked(&self, index: I) -> *const T {
        unsafe { self.backend.get_ptr_unchecked(index.as_usize()) }
    }

    /// Semantically equivalent to
    /// [`PointerIndex::get_mut_ptr`](`PointerIndex::get_mut_ptr`)`(i)` where `i` is
    /// `index.`[`as_usize`](`AsUsize::as_usize`).
    #[inline]
    pub fn get_mut_ptr(&self, index: I) -> *mut T {
        self.backend.get_mut_ptr(index.as_usize())
    }

    /// Semantically equivalent to
    /// [`PointerIndex::get_mut_ptr_unchecked`](`PointerIndex::get_mut_ptr_unchecked`)`(i)` where `i` is
    /// `index.`[`as_usize`](`AsUsize::as_usize`).
    #[inline]
    pub unsafe fn get_mut_ptr_unchecked(&self, index: I) -> *mut T {
        unsafe { self.backend.get_mut_ptr_unchecked(index.as_usize()) }
    }
}
