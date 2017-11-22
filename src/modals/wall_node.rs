pub struct node {
    pub node_id: u32,
    pub from_node_id: u32,
    pub pixle_x: u32,
    pub pixle_y: u32,

    pub num_of_openings: u32,

    pub left_wall: bool,
    pub right_wall: bool,
    pub top_wall: bool,
    pub bot_wall: bool,
}

impl node {
    pub fn new(id: u32, px: u32, py: u32, 
            lwall: u8, rwall: u8, bwall: u8, twall: u8) -> node {
        let node_id = id;
        let mut from_node_id: u32 = 0;
        let pixle_x = px;
        let pixle_y = py;
        let mut num_of_openings = 0;
        let left_wall = false;
        let right_wall = false;
        let top_wall = false;
        let bot_wall = false;

        if lwall == 255 {
            let left_wall = true;
            num_of_openings += 1;
        }
        if rwall == 255 {
            let right_wall = true;
            num_of_openings += 1;
        }
        if bwall == 255 {
            let bot_wall = true;
            num_of_openings += 1;
        }
        if twall == 255 {
            let top_wall = true;
            num_of_openings += 1;
        }

        println!("NOde id: {}", node_id);
        println!("x, y: ({},{})", pixle_x, pixle_y);
        println!("Number of openings: {}", num_of_openings);
        println!("----------------");

        node { node_id, from_node_id, pixle_x, pixle_y, num_of_openings, left_wall, right_wall, bot_wall, top_wall }
    }
}
