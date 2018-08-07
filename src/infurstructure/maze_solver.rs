use modals::node_info::ProcessedMaze;


pub fn solve_maze(maze: ProcessedMaze) {
    search_path(&maze, &maze.starting_node);
}

pub fn search_path(maze: &ProcessedMaze, node_id: &u32) {
    if node_id == &maze.ending_node {
        println!("Hey I solved the maze, I'm going to draw you the path now.");
        // figure out how to draw a path here
    }
    let node = &maze.maze_nodes[*node_id as usize];
    if node.left_wall == true && node.num_of_openings < 3 {
        let move_left = &maze.maze_nodes[transverse(Direction::Left, *node_id, maze.nodes_per_row)];
        //move_left.from_node_id = *node_id;
        //move_left.right_wall = false;
        //move_left.num_of_openings = move_left.num_of_openings - 1;
        search_path(&maze, &move_left.node_id);
    }
    let new_node = &maze.maze_nodes[transverse(Direction::Up, *node_id, maze.nodes_per_row)];
}

pub fn transverse(direction: Direction, node_id: u32, nodes_per_row: u32) -> usize {
    match direction {
        Direction::Left => (node_id - 1) as usize,
        Direction::Right => (node_id + 1) as usize,
        Direction::Up => (node_id - nodes_per_row) as usize,
        Direction::Down => (node_id + nodes_per_row) as usize
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

