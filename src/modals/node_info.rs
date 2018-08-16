use modals::wall_node::Node;

pub struct MazeInfo {
    pub path_length: u8,
    pub wall_length: u8,
    pub maze_openings: Vec<Point>
}

pub struct ProcessedMaze {
    pub starting_node: u32,
    pub ending_node: u32,
    pub nodes_per_row: u32,
    pub path_length: u32,
    pub maze_nodes: Vec<Node>
}

pub struct Point {
    pub x: u32,
    pub y: u32,
    pub on_wall: Direction
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}