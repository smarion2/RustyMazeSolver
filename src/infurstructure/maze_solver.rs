use modals::node_info::ProcessedMaze;
use modals::wall_node::Node;
use std::collections::BinaryHeap;

pub fn solve_maze(mut maze: ProcessedMaze) {
    let mut prio_queue = BinaryHeap::new();
    let start = Node::clone(&maze.maze_nodes[maze.starting_node as usize]);
    prio_queue.push(start);

    while let Some(node) = prio_queue.pop() {
        if maze.ending_node == maze.ending_node {
            // found end draw maze
        }

        if node.left_wall == true {
            let move_left = &mut maze.maze_nodes[transverse(Direction::Left, node.node_id, maze.nodes_per_row)];
            move_left.from_node_id = node.node_id;
            move_left.right_wall = false;
            // calculate f / g / h score
            prio_queue.push(*move_left);
        }
        if node.right_wall == true {
            let move_right = &mut maze.maze_nodes[transverse(Direction::Right, node.node_id, maze.nodes_per_row)];
            move_right.from_node_id = node.node_id;
            move_right.left_wall = false;
            // calculate f / g / h score
            prio_queue.push(*move_right);
        }
        if node.top_wall == true {
            let move_up = &mut maze.maze_nodes[transverse(Direction::Up, node.node_id, maze.nodes_per_row)];
            move_up.from_node_id = node.node_id;
            move_up.bot_wall = false;
            // calculate f / g / h score
            prio_queue.push(*move_up);
        }
        if node.bot_wall == true {
            let move_down = &mut maze.maze_nodes[transverse(Direction::Down, node.node_id, maze.nodes_per_row)];
            move_down.from_node_id = node.node_id;
            move_down.top_wall = false;
            // calculate f / g / h score
            prio_queue.push(*move_down);
        }
    }
}

pub fn transverse(direction: Direction, node_id: u32, nodes_per_row: u32) -> usize {
    match direction {
        Direction::Left => (node_id - 1) as usize,
        Direction::Right => (node_id + 1) as usize,
        Direction::Up => (node_id - nodes_per_row) as usize,
        Direction::Down => (node_id + nodes_per_row) as usize
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

