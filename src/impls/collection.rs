use crate::*;
use std::{ops::Deref, rc::Rc, sync::Arc};

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

macro_rules! impl_trusted_sized {
    ( $( $collection:ty ),+ ) => {
        $(
            unsafe impl<T> TrustedSizedCollection for $collection {
                #[inline(always)]
                fn len(&self) -> usize {
                    self.deref().len()
                }

                #[inline(always)]
                fn is_empty(&self) -> bool {
                    self.deref().is_empty()
                }
            }
        )+
    };
}

impl_trusted_sized!(Box<[T]>, Rc<[T]>, Arc<[T]>);
