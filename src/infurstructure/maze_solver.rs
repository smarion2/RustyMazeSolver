use modals::wall_node::Node;
use modals::node_info::ProcessedMaze;


pub fn solve_maze(maze: ProcessedMaze) {
    
}

pub fn search_path(maze: ProcessedMaze) {
    
}

pub fn transverse(direction: Direction, node_id: u32, nodes_per_row: u32) -> u32 {
    match direction {
        Direction::Left => node_id - 1,
        Direction::Right => node_id + 1,
        Direction::Up => node_id - nodes_per_row,
        Direction::Down => node_id + nodes_per_row
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

