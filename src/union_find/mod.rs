/// Trait for union find data structure. All union find data structures should implement this trait.
///
/// The union find data structure is a data structure that tracks a set of elements partitioned into
/// a number of disjoint (non-overlapping) subsets.
pub trait UnionFind {
    /// Initialize the union find data structure with `size` components
    ///
    /// # Arguments
    ///
    /// * `size`: The number of components
    fn new(size: usize) -> Self;

    /// Connect two components
    ///
    /// # Arguments
    ///
    /// * `p`: The index of the first component to connect
    /// * `q`: The index of the second component to connect
    fn union(&mut self, p: usize, q: usize);

    /// Return the identifier of a component
    ///
    /// * `p`: The index of the component
    fn find(&mut self, p: usize) -> usize;

    /// Return `true` if the components are connected, `false` otherwise
    ///
    /// * `p`: The index of the first component
    /// * `q`: The index of the second component
    fn connected(&mut self, p: usize, q: usize) -> bool;

    /// Return the number of components in the structure
    fn count(&self) -> usize;
}

pub mod quick_find;
pub mod quick_union;
pub mod weighted_quick_union;
pub mod weighted_quick_union_path_compression;
