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

    pub fn find(&self, p: usize) -> usize {
        if p >= self.component_ids.len() {
            panic!("Index {} is out of bounds", p)
        }
        self.component_ids[p]
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn component_count(&self) -> usize {
        self.count
    }
}
