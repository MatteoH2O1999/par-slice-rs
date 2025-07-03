use crate::*;

macro_rules! wrapper_method_doc {
    ($method:path) => {
        concat!(
            "Semantically equivalent to [`",
            stringify!($method),
            "`](`",
            stringify!($method),
            "`)`(index.`[`as_usize`](`AsUsize::as_usize`)`())`."
        )
    };
    ($method:path,$other_params:literal) => {
        concat!(
            "Semantically equivalent to [`",
            stringify!($method),
            "`](`",
            stringify!($method),
            "`)`(index.`[`as_usize`](`AsUsize::as_usize`)`()",
            $other_params,
            ")`."
        )
    };
}

pub struct IndexWrapper<I, T: ?Sized, B> {
    backend: B,
    _marker: std::marker::PhantomData<(I, T)>,
}

impl<T: ?Sized, B: ParView<T>> IndexWrapper<(), T, B> {
    #[inline]
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
    #[doc = wrapper_method_doc!(PointerIndex::get_ptr)]
    #[inline]
    pub fn get_ptr(&self, index: I) -> *const T {
        self.backend.get_ptr(index.as_usize())
    }

    #[doc = wrapper_method_doc!(PointerIndex::get_ptr_unchecked)]
    #[inline]
    pub unsafe fn get_ptr_unchecked(&self, index: I) -> *const T {
        unsafe { self.backend.get_ptr_unchecked(index.as_usize()) }
    }

    #[doc = wrapper_method_doc!(PointerIndex::get_mut_ptr)]
    #[inline]
    pub fn get_mut_ptr(&self, index: I) -> *mut T {
        self.backend.get_mut_ptr(index.as_usize())
    }

    #[doc = wrapper_method_doc!(PointerIndex::get_mut_ptr_unchecked)]
    #[inline]
    pub unsafe fn get_mut_ptr_unchecked(&self, index: I) -> *mut T {
        unsafe { self.backend.get_mut_ptr_unchecked(index.as_usize()) }
    }
}

impl<I: AsUsize, T, B: UnsafeNoRefIndex<T>> IndexWrapper<I, T, B> {
    #[doc = wrapper_method_doc!(UnsafeNoRefIndex::get_value)]
    #[inline]
    pub unsafe fn get_value(&self, index: I) -> T
    where
        T: Copy,
    {
        unsafe { self.backend.get_value(index.as_usize()) }
    }

    #[doc = wrapper_method_doc!(UnsafeNoRefIndex::get_value_unchecked)]
    #[inline]
    pub unsafe fn get_value_unchecked(&self, index: I) -> T
    where
        T: Copy,
    {
        unsafe { self.backend.get_value_unchecked(index.as_usize()) }
    }

    #[doc = wrapper_method_doc!(UnsafeNoRefIndex::set_value, ", value")]
    #[inline]
    pub unsafe fn set_value(&self, index: I, value: T) {
        unsafe {
            self.backend.set_value(index.as_usize(), value);
        }
    }

    #[doc = wrapper_method_doc!(UnsafeNoRefIndex::set_value_unchecked, ", value")]
    #[inline]
    pub unsafe fn set_value_unchecked(&self, index: I, value: T) {
        unsafe {
            self.backend.set_value_unchecked(index.as_usize(), value);
        }
    }
}

impl<I: AsUsize, T, B: UnsafeNoRefChunkIndex<T>> IndexWrapper<I, T, B> {
    #[doc = wrapper_method_doc!(UnsafeNoRefChunkIndex::get_values, ", out")]
    #[inline]
    pub unsafe fn get_values<O: AsMut<[T]>>(&self, index: I, out: O) -> O
    where
        T: Copy,
    {
        unsafe { self.backend.get_values(index.as_usize(), out) }
    }

    #[doc = wrapper_method_doc!(UnsafeNoRefChunkIndex::get_values_unchecked, ", out")]
    #[inline]
    pub unsafe fn get_values_unchecked<O: AsMut<[T]>>(&self, index: I, out: O) -> O
    where
        T: Copy,
    {
        unsafe { self.backend.get_values_unchecked(index.as_usize(), out) }
    }

    #[doc = wrapper_method_doc!(UnsafeNoRefChunkIndex::set_values, ", values")]
    #[inline]
    pub unsafe fn set_values(&self, index: I, values: &[T])
    where
        T: Clone,
    {
        unsafe {
            self.backend.set_values(index.as_usize(), values);
        }
    }

    #[doc = wrapper_method_doc!(UnsafeNoRefChunkIndex::set_values_unchecked, ", values")]
    #[inline]
    pub unsafe fn set_values_unchecked(&self, index: I, values: &[T])
    where
        T: Clone,
    {
        unsafe {
            self.backend.set_values_unchecked(index.as_usize(), values);
        }
    }
}
