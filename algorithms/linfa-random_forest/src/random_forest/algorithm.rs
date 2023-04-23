//! Random forest based on linear decision trees
//!

use linfa_trees::{DecisionTree, Result, SplitQuality};

pub struct TestStruct {
    mask: Vec<bool>,
    nsamples: usize,
}

