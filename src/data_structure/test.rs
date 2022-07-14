use crate::data_structure::fenwick_tree::FenwickTree;

pub fn make_fen() -> FenwickTree<i32> {
    FenwickTree::new(10)
}
