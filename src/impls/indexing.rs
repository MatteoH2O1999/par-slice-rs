use crate::*;

unsafe impl AsUsize for usize {
    #[inline]
    fn as_usize(&self) -> usize {
        *self
    }
}

unsafe impl AsUsize for u8 {
    #[inline]
    fn as_usize(&self) -> usize {
        (*self).into()
    }
}

unsafe impl AsUsize for u16 {
    #[inline]
    fn as_usize(&self) -> usize {
        (*self).into()
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
unsafe impl AsUsize for u32 {
    #[inline]
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[cfg(target_pointer_width = "64")]
unsafe impl AsUsize for u64 {
    #[inline]
    fn as_usize(&self) -> usize {
        *self as usize
    }
}
