use rust_algorithms::union_find::UnionFind;
use rust_algorithms::union_find::weighted_quick_union_path_compression::WeightedQuickUnionPathCompression;

use crate::union_find::common::*;

#[test]
#[should_panic]
fn quick_union_not_zero() {
    WeightedQuickUnionPathCompression::new(0);
}

#[test]
#[should_panic]
fn quick_union_out_of_bounds() {
    let size = 10;
    test_out_of_bounds(&mut WeightedQuickUnionPathCompression::new(size), size);
}

#[test]
fn quick_union_component_count() {
    let size = 10;
    test_component_count(&WeightedQuickUnionPathCompression::new(size), size);
}

#[test]
fn quick_union_union() {
    test_union(&mut WeightedQuickUnionPathCompression::new(5));
}

#[test]
fn quick_union_union_already_connected() {
    test_union_already_connected(&mut WeightedQuickUnionPathCompression::new(5));
}
