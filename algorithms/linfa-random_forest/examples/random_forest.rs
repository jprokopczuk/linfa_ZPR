use ndarray_rand::rand::SeedableRng;
use rand::rngs::SmallRng;
// use std::fmt::Debug;
use linfa_random_forest::{Forest, ForestTwo, Lasowy, Tree};


fn fun_test(las: &dyn Lasowy){
    las.print_forest();
}


fn fun_test2(mask: String) -> Tree {
    Tree {
        mask,
        nsamples: 2,
    }
}
fn main() {
    println!("Main running...");

    // load Iris dataset
    let mut rng = SmallRng::seed_from_u64(42);

    let (_train, _test) = linfa_datasets::iris()
        .shuffle(&mut rng)
        .split_with_ratio(0.8);

    // let simple_forest: Forest::params();

    let forest: Forest = Forest;
    fun_test(&forest);

    
    let forest_two = ForestTwo::forest_two_test();

    fun_test(&forest_two);
    // forest.printForest();

    // let drzewo: Tree;
    let a= fun_test2(String::from("DUPA"));


    // let forest_test= forest_two.create_forest();

    // let test: ForestTwo =  forest_two.forest_two_test();

    // println!("{:#?}", forest_test);
    
    println!("{:#?}", forest_two);

    println!("String: {}", a.mask);
    
    
}
