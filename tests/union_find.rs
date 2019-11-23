use rust_algorithms::union_find::UnionFind;
use rust_algorithms::union_find::quick_find::UnionFindQuickFind;
use rust_algorithms::union_find::quick_union::UnionFindQuickUnion;

#[test]
#[should_panic]
fn quick_find_not_zero() {
    UnionFindQuickFind::new(0);
}

#[test]
#[should_panic]
fn quick_find_out_of_bounds() {
    let size = 10;
    test_out_of_bounds(&UnionFindQuickFind::new(size), size);
}

#[test]
fn quick_find_component_count() {
    let size = 10;
    test_component_count(&UnionFindQuickFind::new(size), size);
}

#[test]
fn quick_find_union() {
    test_union(&mut UnionFindQuickFind::new(5));
}

#[test]
fn quick_find_union_already_connected() {
    test_union_already_connected(&mut UnionFindQuickFind::new(5));
}

#[test]
#[should_panic]
fn quick_union_not_zero() {
    UnionFindQuickUnion::new(0);
}

#[test]
#[should_panic]
fn quick_union_out_of_bounds() {
    let size = 10;
    test_out_of_bounds(&UnionFindQuickUnion::new(size), size);
}

#[test]
fn quick_union_component_count() {
    let size = 10;
    test_component_count(&UnionFindQuickUnion::new(size), size);
}

#[test]
fn quick_union_union() {
    test_union(&mut UnionFindQuickUnion::new(5));
}

#[test]
fn quick_union_union_already_connected() {
    test_union_already_connected(&mut UnionFindQuickFind::new(5));
}

fn test_component_count<T: UnionFind>(uf: &T, size: usize) {
    assert_eq!(size, uf.component_count());
}

fn test_out_of_bounds<T: UnionFind>(uf: &T, size: usize) {
    uf.find(size);
}

fn test_union<T: UnionFind>(uf: &mut T) {
    let size = uf.component_count();
    let (first, second) = (1, 3);
    uf.union(first, second);

    // Check that the components were connected
    assert_eq!(true, uf.connected(first, second));
    // Check that they belong to the same component
    assert_eq!(uf.find(first), uf.find(second));
    // Check that the component count decreased
    assert_eq!(size - 1, uf.component_count());
}

fn test_union_already_connected<T: UnionFind>(uf: &mut T) {
    let (first, second) = (1, 3);
    uf.union(first, second);
    let component_count = uf.component_count();
    uf.union(first, second);
    // Check that the component count did not change
    assert_eq!(component_count, uf.component_count());
}