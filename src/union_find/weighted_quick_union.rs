use crate::union_find::UnionFind;

#[derive(Debug)]
/// The weighted quick-find implementation of the union find data structure. Each component points
/// to its parent, so we have a tree like structure. If two components have the same parent, this
/// means that they belong to the same connected component. This implementation improves the
/// quick-find implementation by keeping track of the size of each tree and linking the root of the
/// smaller tree to the root of the larger tree
pub struct UnionFindWeightedQuickUnion {
    /// Contains the parent of the component
    parents: Vec<usize>,
    /// Contains the size of each component
    size: Vec<usize>,
    /// The number of components
    count: usize,
}

impl UnionFind for UnionFindWeightedQuickUnion {
    /// Initialize the quick union union find data structure with `size` objects. The initialization
    /// has linear complexity (O(N)).
    ///
    /// # Arguments
    ///
    /// * `size`: The number of components
    fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Size should be greater that zero");
        }
        Self { parents: (0..size).collect(), size: vec![1; size], count: size }
    }

    /// Connect two components. It is a logarithmic time operation (O(lgN)).
    ///
    /// # Arguments
    ///
    /// * `p`: The index of the first component to connect
    /// * `q`: The index of the second component to connect
    fn union(&mut self, p: usize, q: usize) {
        let id_p = self.find(p);
        let id_q = self.find(q);

        if id_p == id_q {
            return;
        }

        if self.size[id_p] < self.size[id_q] {
            self.parents[id_p] = id_q;
            self.size[id_q] += self.size[id_p];
        } else {
            self.parents[id_q] = id_p;
            self.size[id_p] += self.size[id_q];
        }

        self.count -= 1;
    }

    /// Return the identifier of a component. It is a linear time operation (O(n)) in the worst case
    /// because trees can get tall
    ///
    /// * `p`: The index of the component
    fn find(&self, p: usize) -> usize {
        if p >= self.parents.len() {
            panic!("Index {} is out of bounds", p)
        }
        let mut id_p = p;
        while id_p != self.parents[id_p] {
            id_p = self.parents[id_p]
        }

        id_p
    }

    /// Return `true` if the components are connected, `false` otherwise. It is a linear time
    /// operation (O(n)).
    ///
    /// * `p`: The index of the first component
    /// * `q`: The index of the second component
    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    /// Return the number of components in the structure. It is a constant time operation (O(1))
    fn count(&self) -> usize {
        self.count
    }
}
