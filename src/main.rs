extern crate mazesolver;

use std::env;
use mazesolver::infurstructure;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
    infurstructure::img_load::open(file);
}