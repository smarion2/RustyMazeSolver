pub struct Node {
    pub node_id: u32,
    pub from_node_id: u32,
    pub pixle_x: u32,
    pub pixle_y: u32,

    pub num_of_openings: u32,

    pub left_wall: bool,
    pub right_wall: bool,
    pub top_wall: bool,
    pub bot_wall: bool
}

impl Node {
    pub fn new(id: u32, px: u32, py: u32, 
            lwall: u8, rwall: u8, bwall: u8, twall: u8) -> Node {
        let pixle_x = px;
        let pixle_y = py;
        let mut openings = 0;
        let mut left = false;
        let mut right = false;
        let mut top = false;
        let mut bot = false;

        if lwall == 255 {
            left = true;
            openings += 1;
        }
        if rwall == 255 {
            right = true;
            openings += 1;
        }
        if bwall == 255 {
            bot = true;
            openings += 1;
        }
        if twall == 255 {
            let top = true;
            openings += 1;
        }

        Node {
            node_id: id,
            from_node_id: 0,
            pixle_x: px,
            pixle_y: py,
            num_of_openings: openings,
            left_wall: left,
            right_wall: right,
            bot_wall: bot,
            top_wall: top
        }
    }
}
