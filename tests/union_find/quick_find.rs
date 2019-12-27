use rust_algorithms::union_find::UnionFind;
use rust_algorithms::union_find::quick_find::QuickFind;

use crate::union_find::common::*;

#[test]
#[should_panic]
fn quick_find_not_zero() {
    QuickFind::new(0);
}

#[test]
#[should_panic]
fn quick_find_out_of_bounds() {
    let size = 10;
    test_out_of_bounds(&QuickFind::new(size), size);
}

#[test]
fn quick_find_component_count() {
    let size = 10;
    test_component_count(&QuickFind::new(size), size);
}

#[test]
fn quick_find_union() {
    test_union(&mut QuickFind::new(5));
}

#[test]
fn quick_find_union_already_connected() {
    test_union_already_connected(&mut QuickFind::new(5));
}
