extern crate mazesolver;

use std::env;
use mazesolver::infurstructure;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
    let img = infurstructure::maze_initializer::open_maze(file);
    let nodes = infurstructure::maze_initializer::create_wall_nodes(&img);    
    infurstructure::maze_solver::solve_maze(nodes);
}