pub struct node {
    node_id: i32,
    pixle_x: i32,
    pixle_y: i32,

    pos_x: i32,
    pos_y: i32,

    num_of_openings: i32,

    left_wall: String,
    right_wall: String,
    top_wall: String,
    bot_wall: String,
}

impl node {
    pub fn new(id: i32, px: i32, py: i32, posx: i32, posy: i32, openings: i32,
            lwall: String, rwall: String, bwall: String, twall: String) {

    }
}
