#[derive(Debug)]
struct UnionFind {

}

impl UnionFind {
    fn new(size: u32) -> UnionFind {
        UnionFind {}
    }

    fn union(self, p: u32, q: u32) {

    }

    fn find(self, p: u32) -> u32 {
        0
    }

    fn connected(self, p: u32, q: u32) -> bool {
        false
    }

    fn components(self) -> u32 {
        0
    }
}

fn main() {
    let uf = UnionFind::new(2);
    println!("{:?}", uf)
}
