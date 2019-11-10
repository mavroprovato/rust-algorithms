mod union_find;

use union_find::quick_find::UnionFindQuickFind;
use union_find::quick_union::UnionFindQuickUnion;

fn main() {
    let connections = [
        (4, 3), (3, 8), (6, 5), (9, 4), (2, 1), (8, 9), (5, 0), (7, 2), (6, 1), (1, 0), (6, 7)
    ];

    println!("\nTesting quick find");
    let mut uf = UnionFindQuickFind::new(10);
    for connection in connections.iter() {
        println!("Connecting {} with {}", connection.0, connection.1);
        println!("Already connected? {}", uf.connected(connection.0, connection.1));
        println!("Before connection: find({})={}, find({})={}", connection.0, uf.find(connection.0),
                 connection.1, uf.find(connection.1));
        uf.union(connection.0, connection.1);
        println!("After connection: find({})={}, find({})={}", connection.0, uf.find(connection.0),
                 connection.1, uf.find(connection.1));
        println!("Component count: {}", uf.component_count());
    }

    println!("\nTesting quick union");
    let mut uf = UnionFindQuickUnion::new(10);
    for connection in connections.iter() {
        println!("Connecting {} with {}", connection.0, connection.1);
        println!("Already connected? {}", uf.connected(connection.0, connection.1));
        println!("Before connection: find({})={}, find({})={}", connection.0, uf.find(connection.0),
                 connection.1, uf.find(connection.1));
        uf.union(connection.0, connection.1);
        println!("After connection: find({})={}, find({})={}", connection.0, uf.find(connection.0),
                 connection.1, uf.find(connection.1));
        println!("Component count: {}", uf.component_count());
    }
}
