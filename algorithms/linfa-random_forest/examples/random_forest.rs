use ndarray_rand::rand::SeedableRng;
use rand::rngs::SmallRng;
use linfa_random_forest::{Forest, ForestInitData, Info, TreeParams};


fn main() {
    println!("Main running...");

    // load Iris dataset
    let mut rng: SmallRng = SmallRng::seed_from_u64(42);

    let (_train, _test) = linfa_datasets::iris()
        .shuffle(&mut rng)
        .split_with_ratio(0.8);

    let init_data= ForestInitData{
        tree_nm: 20,
    };

    let forest = Forest::forest_two_test(&init_data);

    let non_default_params = TreeParams{
        split_quality: Gini,
        max_depth: 100,
        min_weight_split: 1.0,
        min_weight_leaf: 1.0,
    };

    forest.setup_trees(&forest, &non_default_params);

    forest.show_info();
}
