pub struct MazeInfo {
    pub path_length: u8,
    pub wall_length: u8,
    pub maze_openings: Vec<Point>
}

pub struct Point {
    pub x: u32,
    pub y: u32
}