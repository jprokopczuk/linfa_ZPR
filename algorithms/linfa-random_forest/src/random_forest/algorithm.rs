//! Random forest based on linear decision trees
//!

use linfa_trees::{DecisionTree, SplitQuality, DecisionTreeParams};

pub struct TreeParams{
    pub split_quality: SplitQuality,
    pub max_depth: usize,
    pub min_weight_split: f32,
    pub min_weight_leaf: f32,
}

pub struct ForestInitData{
    pub tree_nm: u64,
} 

pub trait Info {
    fn show_info(&self);
}

#[derive(Debug)]
pub struct ForestData{
    forest_name: String,
    forest_vector: Vec<DecisionTreeParams<f64, usize>>,
}

#[derive(Debug)]
pub struct Forest(ForestData);

impl Info for Forest {
    fn show_info(&self){
        println!("{:#?}", self);
    }
}

impl Forest{
    pub fn crete_default_forest(forest_info: &ForestInitData) -> Self{
        let mut temporary_forest_vector: Vec<DecisionTreeParams<f64, usize>> = Vec::new();

        for _ in 1..forest_info.tree_nm{
            temporary_forest_vector.push(DecisionTree::<f64,usize>::params());
        }

        Self(ForestData{
            forest_name: String::from("Forest #1"),
            forest_vector: temporary_forest_vector,
        })
    }

    pub fn setup_trees(self, tree_params: &TreeParams) -> Self{
        for i in 0..self.0.forest_vector.len(){
            self.0.forest_vector[i].split_quality(tree_params.split_quality);
            self.0.forest_vector[i].max_depth(Some(tree_params.max_depth));
            self.0.forest_vector[i].min_weight_split(tree_params.min_weight_split);
            self.0.forest_vector[i].min_weight_leaf(tree_params.min_weight_leaf);
        }
        self
    }
}


//  1. Pass train data.
//  2. Manage train data.
//  2. Train trees.
//  3. Get results.

#[cfg(test)]
mod tests {
    #[test]
    fn autotraits() {
    }
}
