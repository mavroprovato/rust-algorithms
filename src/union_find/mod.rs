pub trait UnionFind {
    fn union(&mut self, p: usize, q: usize);
    fn find(&self, p: usize) -> usize;
    fn connected(&self, p: usize, q: usize) -> bool;
    fn component_count(&self) -> usize;
}

pub mod quick_union;
pub mod quick_find;
