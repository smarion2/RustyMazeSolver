extern crate image;

use std::path::Path;

use modals::wall_node::Node;
use modals::node_info::Point;
use modals::node_info::MazeInfo;
use modals::node_info::ProcessedMaze;
use modals::node_info::Direction;

use self::image::GenericImage;

pub fn open_maze(file: String) -> self::image::DynamicImage {
    // Use the open function to load an image from a PAth.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", im.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    //let fout = &mut File::create(&Path::new(&format!("{}.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    //im.save(fout, image::PNG).unwrap();
    
    return im;
}

pub fn create_wall_nodes(image: &self::image::DynamicImage) -> ProcessedMaze {
    let info = analyze_maze(image);
    let mut nodes: Vec<Node> = Vec::new();
    let (width, height) = image.dimensions();
    println!("width: {:?} height: {:?}", width, height);
    let mut id: u32 = 0;
    let mut x = 0;
    let mut y = 0;
    let node_length: u8;
    x = info.wall_length as u32 - 1;
    y = info.wall_length as u32 - 1;
    node_length = info.wall_length + info.path_length;
    let xbounds = width - node_length as u32; 
    let ybounds = height - node_length as u32;
    while y <= ybounds {
        while x <= xbounds {
            let top_test = image.get_pixel(x + info.path_length as u32 - 1, y).data;
            let bottom_test = image.get_pixel(x + info.path_length as u32 - 1, y + info.path_length as u32 + 1).data;
            let left_test = image.get_pixel(x, y + info.path_length as u32 - 1).data;
            let right_test = image.get_pixel(x + info.path_length as u32 + 1, y + info.path_length as u32 - 1).data;
            
            let n = Node::new(id, x as i32, y as i32, left_test[0], right_test[0], bottom_test[0], top_test[0]);
            nodes.push(n);
            id += 1;
            x += node_length as u32;
        }
        y += node_length as u32;
        x = 1;
    }
    println!("{} Nodes created", nodes.len());
    
    println!("path length: {}", info.path_length);
    println!("wall length: {}", info.wall_length);
    
    // find entrance and exit node ids
    let nodes_per_row = width / node_length as u32;
    let (maze_entrance, entrance_wall) = convert_xy_to_vecpos(&info.maze_openings[0], nodes_per_row, node_length);
    let (maze_exit, exit_wall) = convert_xy_to_vecpos(&info.maze_openings[info.path_length as usize], nodes_per_row, node_length);
    println!("pint1: ({},{})", &info.maze_openings[0].x, &info.maze_openings[0].y);
    println!("point2: ({},{})", &info.maze_openings[info.path_length as usize].x, &info.maze_openings[info.path_length as usize].y);
    
    println!("maze entrance id: {}", maze_entrance);
    println!("maze exit id: {}", maze_exit);

    {
        let start = &mut nodes[maze_entrance as usize];
        println!("entrance_wall {:?}", entrance_wall);
        match entrance_wall {
            Direction::Left => start.left_wall = false,
            Direction::Right => start.right_wall = false,
            Direction::Up => start.top_wall = false,
            Direction::Down => start.bot_wall = false
        }
    }
    {
        let end = &mut nodes[maze_exit as usize];
        match exit_wall {
            Direction::Left => end.left_wall = false,
            Direction::Right => end.right_wall = false,
            Direction::Up => end.top_wall = false,
            Direction::Down => end.bot_wall = false
        }
    }

    ProcessedMaze { starting_node: maze_entrance, ending_node: maze_exit, nodes_per_row: nodes_per_row, maze_nodes: nodes }
}
// need to decide how much to analyze, i dont think going through the entire maze is necessary 
// maybe just look around all the edges for opening and closing and a few lines?
fn analyze_maze(image: &self::image::DynamicImage) -> MazeInfo { 
    println!("Maze loaded scanning maze now...");
    let (img_width, img_height) = image.dimensions();       
    let wall_color = image.get_pixel(0, 0).data;
    let mut path_width: u8 = 0;
    let mut openings: Vec<Point> = Vec::new();    
    let is_path_uniform = false;

    let (path_length, mut maze_openings) = check_for_openings(&image, 0, true, Direction::Left, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        openings.append(&mut maze_openings);
    }
    let (path_length, mut maze_openings) = check_for_openings(&image, img_width - 1, true, Direction::Right, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        openings.append(&mut maze_openings);
    }

    let (path_length, mut maze_openings) = check_for_openings(&image, 1, false, Direction::Up, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        openings.append(&mut maze_openings);
    }
    let (path_length, mut maze_openings) = check_for_openings(&image, img_height - 1, false, Direction::Down, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        openings.append(&mut maze_openings);
    }
    if !is_path_uniform {
        println!("Path is unstable!");
    }
    let wall_length = check_for_wall_length(&image, wall_color);    
    MazeInfo { path_length: path_width, wall_length: wall_length, maze_openings: openings }
}

fn check_path_length(current: u8, new_length: u8) -> (u8, bool) {
    if current == 0 {
        return (new_length, false)
    } else if current == new_length {
        return (new_length, true)
    } else if new_length < current {
        return (new_length, false)
    }
    (0, false)
}

fn check_for_wall_length(image: &self::image::DynamicImage, wall_color: [u8; 4]) -> u8 {
    let (img_width, img_height) = image.dimensions();
    let mut wall_length: u8 = 0;
    let middle_horizontal: u32 = img_height / 2;
    let middle_verticle: u32 = img_width / 2;
    for y in 0..img_width {
        let mut length: u8 = 0;
        if image.get_pixel(middle_horizontal, y).data == wall_color {
            while image.get_pixel(middle_horizontal, y + length as u32).data == wall_color {
                length += 1
            }
            if length < wall_length || wall_length == 0 {
                wall_length = length;
            }
        } else if y > middle_verticle {
            break;
        }
    }

    wall_length + 1
}

fn convert_xy_to_vecpos(point: &Point, nodes_per_row: u32, node_length: u8) -> (u32, Direction) {
    //println!("nodes per row: {:?}", nodes_per_row);
    //println!("node length: {:?}", node_length);
    let mut x_pos = (point.x) / node_length as u32;
    if x_pos == nodes_per_row {
        x_pos = x_pos - 1;
    }
    //println!("x_pos: {:?}", x_pos);
    let mut y_pos = (point.y) / node_length as u32;
    if y_pos == nodes_per_row {
        y_pos = y_pos - 1;
    }
    //println!("y_pos: {:?}", y_pos);
    ((y_pos * nodes_per_row + x_pos), point.on_wall)
}

fn check_for_openings(image: &self::image::DynamicImage, wall_pos: u32, is_vertical: bool, wall: Direction, wall_color: [u8; 4]) -> (u8, Vec<Point>) {
    let (img_width, img_height) = image.dimensions();
    let mut points: Vec<Point> = Vec::new();
    let mut path_length: u8 = 0;
    if is_vertical {
        for y in 1..img_height {
            let pixle_color = image.get_pixel(wall_pos, y).data;            
            if pixle_color != wall_color {
                //println!("wall opening point x: {:?} y: {:?}", wall_pos, y);
                points.push(Point{x: wall_pos, y: y, on_wall: wall});
                let mut length: u8 = 0;
                while pixle_color == image.get_pixel(wall_pos, y + length as u32).data {
                    length += 1;
                }
                if path_length == 0 {
                   path_length = length;
                }
            }
        } 
    }
    else {
        for x in 1..img_width {
            let pixle_color = image.get_pixel(x, wall_pos).data;
            if pixle_color != wall_color {
                //println!("wall opening point x: {:?} y: {:?}", x, wall_pos);
                points.push(Point{x: x, y: wall_pos, on_wall: wall });
                let mut length: u8 = 0;
                while pixle_color == image.get_pixel(x + length as u32, wall_pos).data {
                    length += 1;
                }
                if path_length == 0 {
                    path_length = length;
                }                
            }
        }
    }
    (path_length, points)
}

