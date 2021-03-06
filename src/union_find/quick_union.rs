use crate::union_find::UnionFind;

#[derive(Debug)]
/// The quick-find implementation of the union find data structure. Each component points to its
/// parent, so we have a tree like structure. If two components have the same parent, this means
/// that they belong to the same connected component.
pub struct QuickUnion {
    /// Contains the parent of the component
    parents: Vec<usize>,
    /// The number of components
    count: usize,
}

impl UnionFind for QuickUnion {
    /// Initialize the quick union union-find data structure with `size` objects. The initialization
    /// has linear complexity (O(N)).
    ///
    /// # Arguments
    ///
    /// * `size`: The number of components
    fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Size should be greater that zero");
        }
        Self { parents: (0..size).collect(), count: size }
    }

    /// Connect two components. It is a linear time operation (O(N)) in the worst case because trees
    /// can get tall.
    ///
    /// # Arguments
    ///
    /// * `p`: The index of the first component to connect
    /// * `q`: The index of the second component to connect
    fn union(&mut self, p: usize, q: usize) {
        let id_p = self.find(p);
        let id_q = self.find(q);

        if id_p != id_q {
            self.parents[id_p] = self.find(id_q);
            self.count -= 1;
        }
    }

    /// Return the identifier of a component. It is a linear time operation (O(N)) in the worst case
    /// because trees can get tall.
    ///
    /// * `p`: The index of the component
    fn find(&mut self, p: usize) -> usize {
        if p >= self.parents.len() {
            panic!("Index {} is out of bounds", p)
        }
        let mut id_p = p;
        while id_p != self.parents[id_p] {
            id_p = self.parents[id_p]
        }

        id_p
    }

    /// Return the number of components in the structure. It is a constant time operation (O(1))
    fn count(&self) -> usize {
        self.count
    }
}
