#[derive(Debug)]
pub struct UnionFindQuickFind {
    component_ids: Vec<usize>,
    count: usize
}

impl UnionFindQuickFind {
    pub fn new(size: usize) -> UnionFindQuickFind {
        UnionFindQuickFind { component_ids: (0..size).collect(), count: size }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        // Get the ids
        let id_p = self.component_ids[p];
        let id_q = self.component_ids[q];
        // Check if they are already in the same component
        if id_p == id_q {
            return;
        }
        // Connect the components
        for i in 0..self.component_ids.len() {
            if self.component_ids[i] == id_p {
                self.component_ids[i] = id_q
            }
        }
        // Decrease the component count
        self.count -= 1;
    }

    pub fn find(&self, p: usize) -> usize {
        self.component_ids[p]
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.component_ids[p] == self.component_ids[q]
    }

    pub fn component_count(&self) -> usize {
        self.count
    }
}
