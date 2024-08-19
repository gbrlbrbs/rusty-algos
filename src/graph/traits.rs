use std::hash::Hash;
use std::fmt::Debug;
use std::u16;


pub unsafe trait IndexType: Copy + Default + Hash + Ord + Debug + 'static {
    fn new(x: usize) -> Self;
    fn max() -> Self;
    fn index(&self) -> usize;
}

unsafe impl IndexType for usize {
    index_impl!(usize);
}

unsafe impl IndexType for u32 {
    index_impl!(u32);
}

unsafe impl IndexType for u16 {
    index_impl!(u16);
}