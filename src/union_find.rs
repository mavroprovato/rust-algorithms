#[derive(Debug)]
pub struct UnionFindQuickFind {
    ids: Vec<usize>,
    count: usize
}

impl UnionFindQuickFind {
    pub fn new(size: usize) -> UnionFindQuickFind {
        UnionFindQuickFind { ids: (0..size).collect(), count: size }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        // Get the ids
        let id_p = self.ids[p];
        let id_q = self.ids[q];
        // Check if they are already in the same component
        if id_p == id_q {
            return;
        }
        // Connect the components
        for i in 0..self.ids.len() {
            if self.ids[i] == id_p {
                self.ids[i] = id_q
            }
        }
        // Decrease the component count
        self.count -= 1;
    }

    pub fn find(&self, p: usize) -> usize {
        self.ids[p]
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.ids[p] == self.ids[q]
    }

    pub fn component_count(&self) -> usize {
        self.count
    }
}
