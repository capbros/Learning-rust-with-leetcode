use std::fmt::Debug;

pub fn equals_unordered<T: PartialEq + Ord + Debug>(mut left: Vec<T>, mut right: Vec<T>) {
    left.sort();
    right.sort();
    assert_eq!(left, right);
}