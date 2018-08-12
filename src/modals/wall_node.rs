pub struct Node {
    pub node_id: u32,
    pub from_node_id: u32,
    pub pixle_x: u32,
    pub pixle_y: u32,

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
        let mut left = false;
        let mut right = false;
        let mut top = false;
        let mut bot = false;

        if lwall == 255 {
            left = true;
        }
        if rwall == 255 {
            right = true;
        }
        if bwall == 255 {
            bot = true;
        }
        if twall == 255 {
            top = true;
        }

        Node {
            node_id: id,
            from_node_id: 0,
            pixle_x: px,
            pixle_y: py,
            left_wall: left,
            right_wall: right,
            bot_wall: bot,
            top_wall: top
        }
    }

    pub fn clone(node: &Node) -> Node {
        Node {
            node_id: node.node_id,
            from_node_id: node.from_node_id,
            pixle_x: node.pixle_x,
            pixle_y: node.pixle_y,
            left_wall: node.left_wall,
            right_wall: node.right_wall,
            bot_wall: node.bot_wall,
            top_wall: node.top_wall
        }
    }
}
