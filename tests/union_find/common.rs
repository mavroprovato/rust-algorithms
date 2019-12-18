use rust_algorithms::union_find::UnionFind;

pub fn test_component_count<T: UnionFind>(uf: &T, size: usize) {
    assert_eq!(size, uf.count());
}

pub fn test_out_of_bounds<T: UnionFind>(uf: &T, size: usize) {
    uf.find(size);
}

pub fn test_union<T: UnionFind>(uf: &mut T) {
    let size = uf.count();
    let (first, second) = (1, 3);
    uf.union(first, second);

    // Check that the components were connected
    assert_eq!(true, uf.connected(first, second));
    // Check that they belong to the same component
    assert_eq!(uf.find(first), uf.find(second));
    // Check that the component count decreased
    assert_eq!(size - 1, uf.count());
}

pub fn test_union_already_connected<T: UnionFind>(uf: &mut T) {
    let (first, second) = (1, 3);
    uf.union(first, second);
    let component_count = uf.count();
    uf.union(first, second);
    // Check that the component count did not change
    assert_eq!(component_count, uf.count());
}