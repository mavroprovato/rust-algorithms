#[derive(Debug)]
struct UnionFind {

}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {}
    }

    fn union(self, p: usize, q: usize) {

    }

    fn find(self, p: usize) -> usize {
        0
    }

    fn connected(self, p: usize, q: usize) -> bool {
        false
    }

    fn components(self) -> usize {
        0
    }
}

fn main() {
    let uf = UnionFind::new(2);
    println!("{:?}", uf)
}
