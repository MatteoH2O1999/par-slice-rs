pub unsafe trait UnsafeDataRaceAccess<T: ?Sized> {
    unsafe fn get(&self, index: usize) -> T
    where
        T: Copy;

    unsafe fn get_unchecked(&self, index: usize) -> T
    where
        T: Copy;

    unsafe fn set(&self, index: usize, value: T)
    where
        T: Sized;

    unsafe fn set_unchecked(&self, index: usize, value: T)
    where
        T: Sized;
}

pub unsafe trait UnsafeDataRaceChunkAccess<T> {
    unsafe fn get(&self, index: usize) -> Box<[T]>
    where
        T: Copy;

    unsafe fn get_unchecked(&self, index: usize) -> Box<[T]>
    where
        T: Copy;

    unsafe fn set(&self, index: usize, value: &[T])
    where
        T: Clone;

    unsafe fn set_unchecked(&self, index: usize, value: &[T])
    where
        T: Clone;
}
