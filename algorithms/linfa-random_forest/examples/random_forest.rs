use ndarray_rand::rand::SeedableRng;
use rand::rngs::SmallRng;

use linfa_random_forest::{Forest};

fn main() {
    println!("Main running...");

    // load Iris dataset
    let mut rng = SmallRng::seed_from_u64(42);

    let (train, test) = linfa_datasets::iris()
        .shuffle(&mut rng)
        .split_with_ratio(0.8);

    let simple_forest: Forest::params();
 
    
}
