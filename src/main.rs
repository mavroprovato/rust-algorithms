mod union_find;

use union_find::UnionFind;

fn main() {
    let uf = UnionFind::new(2);
    println!("{:?}", uf)
}
