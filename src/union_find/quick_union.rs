#[derive(Debug)]
pub struct UnionFindQuickUnion {
    parents: Vec<usize>,
    count: usize
}

impl UnionFindQuickUnion {
    pub fn new(size: usize) -> UnionFindQuickUnion {
        UnionFindQuickUnion { parents: (0..size).collect(), count: size }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let id_p = self.find(p);
        let id_q = self.find(q);

        if id_p != id_q {
            self.parents[id_p] = self.find(id_q);
            self.count -= 1;
        }
    }

    pub fn find(&self, p: usize) -> usize {
        if p >= self.parents.len() {
            panic!("Index {} is out of bounds", p)
        }
        let mut id_p = p;
        while id_p != self.parents[id_p] {
            id_p = self.parents[id_p]
        }

        id_p
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn component_count(&self) -> usize {
        self.count
    }
}

