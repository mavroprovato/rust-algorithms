use crate::union_find::UnionFind;

#[derive(Debug)]
/// The quick-find implementation of the union find data structure. We keep track of each component
/// identifier, so if two components have the same identifier they belong to the same component.
pub struct UnionFindQuickFind {
    /// The component identifiers. Two components are connected if they have the same component
    /// identifier
    component_ids: Vec<usize>,
    /// The number of components
    count: usize,
}

impl UnionFind for UnionFindQuickFind {
    /// Initialize the quick find union find data structure with `size` objects
    ///
    /// # Arguments
    ///
    /// * `size`: The number of components
    fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Size should be greater that zero");
        }
        Self { component_ids: (0..size).collect(), count: size }
    }

    /// Connect two components. It is a linear time operation (O(n))
    ///
    /// # Arguments
    ///
    /// * `p`: The index of the first component to connect
    /// * `q`: The index of the second component to connect
    fn union(&mut self, p: usize, q: usize) {
        // Get the ids
        let id_p = self.find(p);
        let id_q = self.find(q);
        // Check if they are already in the same component
        if id_p == id_q {
            return;
        }
        // Connect the components
        for i in 0..self.component_ids.len() {
            if self.find(i) == id_p {
                self.component_ids[i] = id_q
            }
        }
        // Decrease the component count
        self.count -= 1;
    }

    /// Return the identifier of a component. It is a constant time operation (O(1))
    ///
    /// * `p`: The index of the component
    fn find(&self, p: usize) -> usize {
        if p >= self.component_ids.len() {
            panic!("Index {} is out of bounds", p)
        }

        self.component_ids[p]
    }

    /// Return `true` if the components are connected, `false` otherwise. It is a constant time
    /// operation (O(1)).
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