use crate::union_find::UnionFind;

#[derive(Debug)]
pub struct UnionFindQuickUnion {
    parents: Vec<usize>,
    count: usize
}

impl UnionFindQuickUnion {
    pub fn new(size: usize) -> Self {
        UnionFindQuickUnion { parents: (0..size).collect(), count: size }
    }
}

impl UnionFind for UnionFindQuickUnion {
    fn union(&mut self, p: usize, q: usize) {
        let id_p = self.find(p);
        let id_q = self.find(q);

        if id_p != id_q {
            self.parents[id_p] = self.find(id_q);
            self.count -= 1;
        }
    }

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

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn component_count(&self) -> usize {
        self.count
    }
}

