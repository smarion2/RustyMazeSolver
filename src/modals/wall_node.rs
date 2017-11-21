pub struct node {
    pub node_id: i32,
    pub from_node_id: i32,
    pub pixle_x: i32,
    pub pixle_y: i32,

    pub pos_x: i32,
    pub pos_y: i32,

    pub num_of_openings: i32,

    pub left_wall: String,
    pub right_wall: String,
    pub top_wall: String,
    pub bot_wall: String,
}

impl node {
    pub fn new(id: i32, px: i32, py: i32, posx: i32, posy: i32, openings: i32,
            lwall: String, rwall: String, bwall: String, twall: String) -> node {
        let node_id = id;
        let from_node_id: i32 = 0;
        let pixle_x = px;
        let pixle_y = py;
        let pos_x = posx;
        let pos_y = posy;
        let num_of_openings = openings;
        let left_wall = lwall;
        let right_wall = rwall;
        let bot_wall = bwall;
        let top_wall = twall;


        node { node_id, from_node_id, pixle_x, pixle_y, pos_x, pos_y, num_of_openings, left_wall, right_wall, bot_wall, top_wall }
    }
}
