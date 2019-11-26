use rust_algorithms::union_find::UnionFind;
use rust_algorithms::union_find::quick_find::UnionFindQuickFind;

use crate::union_find::common::*;

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
