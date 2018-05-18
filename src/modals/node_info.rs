pub struct maze_info {
    pub path_length: u8,
    pub wall_length: u8,
    pub maze_openings: Vec<point>
}

pub struct point {
    pub x: u32,
    pub y: u32
}