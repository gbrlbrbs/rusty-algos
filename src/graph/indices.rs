use super::traits::IndexType;

pub type DefaultIndex = u16;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeIdx<Ix = DefaultIndex>(Ix);

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EdgeIdx<Ix = DefaultIndex>(Ix);

unsafe impl <Ix: IndexType> IndexType for NodeIdx<Ix> {
    #[inline]
    fn new(x: usize) -> Self {
        NodeIdx(IndexType::new(x))
    }

    #[inline]
    fn index(&self) -> usize {
        self.0.index()
    }

    #[inline]
    fn max() -> Self {
        NodeIdx(<Ix as IndexType>::max())
    }
}

impl <Ix: IndexType> NodeIdx<Ix> {
    #[inline]
    pub fn end() -> Self {
        <NodeIdx<Ix> as IndexType>::max()
    }
}

unsafe impl <Ix: IndexType> IndexType for EdgeIdx<Ix> {
    #[inline]
    fn new(x: usize) -> Self {
        EdgeIdx(IndexType::new(x))
    }

    #[inline]
    fn index(&self) -> usize {
        self.0.index()
    }

    #[inline]
    fn max() -> Self {
        EdgeIdx(<Ix as IndexType>::max())
    }
}

impl <Ix: IndexType> EdgeIdx<Ix> {
    #[inline]
    pub fn end() -> Self {
        <EdgeIdx<Ix> as IndexType>::max()
    }
}

impl <Ix: IndexType> From<Ix> for NodeIdx<Ix> {
    fn from(value: Ix) -> Self {
        NodeIdx(value)
    }
}

impl <Ix: IndexType> From<Ix> for EdgeIdx<Ix> {
    fn from(value: Ix) -> Self {
        EdgeIdx(value)
    }
}