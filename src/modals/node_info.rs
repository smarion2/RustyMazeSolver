pub struct info {
    pub height: u8,
    pub width: u8,
    pub path_color: [u8; 3],
    pub maze_opening: point,
    pub maze_ending: point
}

pub struct point {
    pub x: u32,
    pub y: u32
}