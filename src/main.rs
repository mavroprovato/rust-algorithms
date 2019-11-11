mod union_find;

use union_find::UnionFind;
use union_find::quick_find::UnionFindQuickFind;
use union_find::quick_union::UnionFindQuickUnion;

fn main() {
    let connections = [
        (4, 3), (3, 8), (6, 5), (9, 4), (2, 1), (8, 9), (5, 0), (7, 2), (6, 1), (1, 0), (6, 7)
    ];

    println!("Testing quick find");
    let mut uf = UnionFindQuickFind::new(10);
    test_union_find(&connections, &mut uf);

    println!("\n");

    println!("Testing quick union");
    let mut uf = UnionFindQuickUnion::new(10);
    test_union_find(&connections, &mut uf);
}

fn test_union_find(connections: &[(usize, usize)], uf: &mut dyn UnionFind) {
    for &(first, second) in connections.iter() {
        let connected_before = uf.connected(first, second);
        let id_first_before = uf.find(first);
        let id_second_before = uf.find(second);
        let component_count_before = uf.component_count();
        if id_first_before == id_second_before && !connected_before {
            panic!("Ids where the same but the components where not connected");
        }

        println!("Connecting {} with {}", first, second);
        uf.union(first, second);

        let connected_after = uf.connected(first, second);
        let id_first_after = uf.find(first);
        let id_second_after = uf.find(second);
        let component_count_after = uf.component_count();
        if !connected_after {
            panic!("Components are not connected");
        }
        if id_first_after != id_second_after {
            panic!("Ids are not the same");
        }

        if connected_before && component_count_before != component_count_after {
            panic!("Component count changed even though the sites where connected");
        }
        if !connected_before && component_count_before != component_count_after + 1 {
            panic!("Component count did not decrease even though the sites where not connected");
        }
    }
}
