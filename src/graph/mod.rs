// Graph data structure implementation
pub mod traits;
pub mod indices;
use std::marker::PhantomData;

use indices::*;
use traits::IndexType;

pub enum Directed {}

pub enum Undirected {}

pub enum Direction {
    Incoming = 0,
    Outgoing = 1,
}

impl Direction {
    #[inline]
    pub fn index(self) -> usize {
        (self as usize) & 0x1
    }
}

const DIRECTIONS: [Direction; 2] = [Direction::Incoming, Direction::Outgoing];

#[derive(Debug)]
pub struct Edge<E, Ix = DefaultIndex> {
    pub data: E,
    next: [EdgeIdx<Ix>; 2],
    /// Incoming (target) and outgoing (source) nodes.
    nodes: [NodeIdx<Ix>; 2]
}

impl <E, Ix: IndexType> Edge<E, Ix> {
    pub fn next_edge(&self, dir: Direction) -> EdgeIdx<Ix> {
        self.next[dir.index()]
    }

    pub fn node(&self, dir: Direction) -> NodeIdx<Ix> {
        self.nodes[dir.index()]
    }
}

#[derive(Debug)]
pub struct Node<N, Ix = DefaultIndex> {
    pub data: N,
    next: [EdgeIdx<Ix>; 2]
}

impl <N, Ix: IndexType> Node<N, Ix> {
    pub fn next_edge(&self, dir: Direction) -> EdgeIdx<Ix> {
        self.next[dir.index()]
    }
}

impl <N, Ix: IndexType> Clone for Node<N, Ix> 
where 
    N: Clone,
    Ix: Copy
{
    clone_fields!(Node, data, next);
}

impl <E, Ix: IndexType> Clone for Edge<E, Ix>
where
    E: Clone,
    Ix: Copy
{
    clone_fields!(Edge, data, next, nodes);
}



pub struct BaseGraph <N, E, Ix = DefaultIndex, Type=Undirected> {
    pub nodes: Vec<Node<N, Ix>>,
    pub edges: Vec<Edge<E, Ix>>,
    ty: PhantomData<Type>,
    edge_type
}

pub type Graph<N, E, Ix = DefaultIndex> = BaseGraph<N, E, Ix>;

pub type DiGraph<N, E, Ix = DefaultIndex> = BaseGraph<N, E, Ix, Directed>;
