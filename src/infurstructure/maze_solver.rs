use modals::node_info::ProcessedMaze;
use modals::node_info::Direction;
use modals::wall_node::Node;
use std::collections::BinaryHeap;
use std::num;

pub fn solve_maze(mut maze: ProcessedMaze) {
    println!("Starting to solve");
    let mut prio_queue = BinaryHeap::new();
    let start = Node::clone(&maze.maze_nodes[(maze.starting_node -1) as usize]);
    let end = Node::clone(&maze.maze_nodes[maze.ending_node as usize]);
    prio_queue.push(start);
    let mut nodes_explored: u32 = 0;
    while let Some(node) = prio_queue.pop() {
        nodes_explored += 1;
        //println!("node: {} top:{} bot:{} left:{} right:{}", node.node_id, node.top_wall, node.bot_wall, node.left_wall, node.right_wall);
        if node.node_id == maze.ending_node {
            println!("SOLVED MAZE!~ Explored {} nodes", nodes_explored);
            break;
        }

        if node.left_wall == true {
            let move_left = &mut maze.maze_nodes[transverse(Direction::Left, node.node_id, maze.nodes_per_row)];
            move_left.from_node_id = node.node_id;
            move_left.right_wall = false;
            // calculate f / g / h score
            move_left.g = node.g + 1;
            move_left.h = (move_left.pixle_x - end.pixle_x).abs() + (move_left.pixle_y - end.pixle_y).abs();
            move_left.f = move_left.g + move_left.h as u32;
            prio_queue.push(*move_left);
        }
        if node.right_wall == true {
            let move_right = &mut maze.maze_nodes[transverse(Direction::Right, node.node_id, maze.nodes_per_row)];
            move_right.from_node_id = node.node_id;
            move_right.left_wall = false;
            // calculate f / g / h score
            move_right.g = node.g + 1;
            move_right.h = (move_right.pixle_x - end.pixle_x).abs() + (move_right.pixle_y - end.pixle_y).abs();
            move_right.f = move_right.g + move_right.h as u32;
            prio_queue.push(*move_right);
        }
        if node.top_wall == true {
            let move_up = &mut maze.maze_nodes[transverse(Direction::Up, node.node_id, maze.nodes_per_row)];
            move_up.from_node_id = node.node_id;
            move_up.bot_wall = false;
            // calculate f / g / h score
            move_up.g = node.g + 1;
            move_up.h = (move_up.pixle_x - end.pixle_x).abs() + (move_up.pixle_y - end.pixle_y).abs();
            move_up.f = move_up.g + move_up.h as u32;
            prio_queue.push(*move_up);
        }
        if node.bot_wall == true {
            let move_down = &mut maze.maze_nodes[transverse(Direction::Down, node.node_id, maze.nodes_per_row)];
            move_down.from_node_id = node.node_id;
            move_down.top_wall = false;
            // calculate f / g / h score
            move_down.g = node.g + 1;
            move_down.h = (move_down.pixle_x - end.pixle_x).abs() + (move_down.pixle_y - end.pixle_y).abs();
            move_down.f = move_down.g + move_down.h as u32;
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



