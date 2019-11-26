use rust_algorithms::union_find::UnionFind;
use rust_algorithms::union_find::quick_union::UnionFindQuickUnion;

use crate::union_find::common::*;

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
    test_union_already_connected(&mut UnionFindQuickUnion::new(5));
}
