use ndarray_rand::rand::SeedableRng;
use rand::rngs::SmallRng;
use linfa_random_forest::{Forest, ForestInitData, Info, TreeParams};
use linfa_trees::{SplitQuality};

fn main() {
    println!("Main running...");

    // load Iris dataset
    let mut rng: SmallRng = SmallRng::seed_from_u64(42);

    let (_train, _test) = linfa_datasets::iris()
        .shuffle(&mut rng)
        .split_with_ratio(0.8);


    // Set tree settings
    let init_data= ForestInitData{
        tree_nm: 20,
    };

    // Set all trees params
    let non_default_params = TreeParams{
        split_quality: SplitQuality::Gini,
        max_depth: 100,
        min_weight_split: 1.0,
        min_weight_leaf: 1.0,
    };

    // Get forest object
    let forest = Forest::crete_default_forest(&init_data)
    .setup_trees(&non_default_params);

    // Print forest structure
    forest.show_info();
}
