//! Random forest based on linear decision trees
//!

use linfa_trees::{DecisionTree, Result, SplitQuality, DecisionTreeParams};
// use codec::vec;

pub struct Tree {
    pub mask: String,
    pub nsamples: u8,
}

// pub struct Forest{
//     forest: u32,
//     // forest: Vec<Tree>,
// }

pub trait ForestManager {
    fn params() -> Forest;
}

// impl ForestManager for Forest {
//     /// Defaults are provided if the optional parameters are not specified:
//     /// * `split_quality = SplitQuality::Gini`
//     /// TODO
//     /// * `max_depth = None`
//     /// * `min_weight_split = 2.0`
//     /// * `min_weight_leaf = 1.0`
//     /// * `min_impurity_decrease = 0.00001`
//     // Violates the convention that new should return a value of type `Self`
//     // #[allow(clippy::new_ret_no_self)]
//     fn params() -> Forest {
//         Forest { forest: (32), }
//     }
// }
pub trait Lasowy {
    fn print_forest(&self);
}


pub struct Forest;
// {
//     forest_name: String,
//     email: u64,
// }

impl Lasowy for Forest {
    fn print_forest(&self){
        println!("I'm in forest!");
    }
}

#[derive(Debug)]
pub struct ForestEx{
    forest_name: String,
    email: u64,
    vector: DecisionTreeParams<f64, usize>,
}

#[derive(Debug)]
pub struct ForestTwo(ForestEx);

impl Lasowy for ForestTwo {
    fn print_forest(&self){
        println!("I'm in forestTwo!");
    }
}

impl ForestTwo{
    pub fn forest_two_test(&self) -> Self{
        Self(ForestEx{
            forest_name: String::from("DUPA INIT"),
            email: (453),
            vector: self.create_forest(),
        })
    }

    pub fn create_forest(&self) -> DecisionTreeParams<f64, usize> {
        DecisionTree::<f64,usize>::params()
    }

    pub fn create_forest_vector(&self){

    }
}



// DecisionTree::params()
//         .split_quality(SplitQuality::Gini)
//         .max_depth(Some(100))
//         .min_weight_split(1.0)
//         .min_weight_leaf(1.0)
//         .fit(&train)?;

// impl Forest{
//     // Return test
//     pub fn new() -> Self {
//         Forest { forest: (32) }
//     }
// }



// 1. Init one tree
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


#[cfg(test)]
mod tests {
    #[test]
    fn autotraits() {
    }
}
