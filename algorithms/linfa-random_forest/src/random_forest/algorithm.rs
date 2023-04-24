//! Random forest based on linear decision trees
//!

use linfa_trees::{DecisionTree, Result, SplitQuality};

pub struct Tree {
    mask: Vec<bool>,
    nsamples: usize,
}

pub struct Forest{
    forest: u32,
    // forest: Vec<Tree>,
}


impl Forest {
    /// Defaults are provided if the optional parameters are not specified:
    /// * `split_quality = SplitQuality::Gini`
    /// TODO
    /// * `max_depth = None`
    /// * `min_weight_split = 2.0`
    /// * `min_weight_leaf = 1.0`
    /// * `min_impurity_decrease = 0.00001`
    // Violates the convention that new should return a value of type `Self`
    #[allow(clippy::new_ret_no_self)]
    pub fn params() -> Self {
        Forest { forest: (32), }
    }
}

// impl Forest{
//     // Return test
//     pub fn new() -> Self {
//         Forest { forest: (32) }
//     }
// }


// TODO:
//  1. Get train data.
//  2. Get number of trees in forest.
//  3. Get max depth.
//  4. Get all tree settings


// let gini_model = DecisionTree::params()
// .split_quality(SplitQuality::Gini)
// .max_depth(Some(100))
// .min_weight_split(1.0)
// .min_weight_leaf(1.0)
// .fit(&train)?;
