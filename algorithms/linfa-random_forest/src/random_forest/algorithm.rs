//! Random forest based on linear decision trees
//!

use linfa_trees::{DecisionTree, Result, SplitQuality, DecisionTreeParams};

pub struct ForestInitData{
    pub tree_nm: u64,
} 

pub trait Info {
    fn show_info(&self);
}

#[derive(Debug)]
pub struct ForestData{
    forest_name: String,
    vector: Vec<DecisionTreeParams<f64, usize>>,
}

#[derive(Debug)]
pub struct Forest(ForestData);

impl Info for Forest {
    fn show_info(&self){
        // println!("I'm in forestTwo!");
        println!("{:#?}", self);
    }
}

impl Forest{
    pub fn forest_two_test(forest_info: &ForestInitData) -> Self{
        let mut values: Vec<DecisionTreeParams<f64, usize>> = Vec::new();

        for _ in 1..forest_info.tree_nm{
            values.push(DecisionTree::<f64,usize>::params());
        }

        Self(ForestData{
            forest_name: String::from("Forest #1"),
            vector: values,
        })
    }

    pub fn create_forest(&self) -> DecisionTreeParams<f64, usize> {
        DecisionTree::<f64,usize>::params()
    }
}


// 1. Init one tree
// TODO:
//  1. Get train data.
//  2. Get number of trees in forest.
//  3. Get max depth.
//  4. Get all tree settings


#[cfg(test)]
mod tests {
    #[test]
    fn autotraits() {
    }
}
