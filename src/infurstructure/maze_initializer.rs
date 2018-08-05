extern crate image;

use std::path::Path;

use modals::wall_node::Node;
use modals::node_info::Point;
use modals::node_info::MazeInfo;
use modals::node_info::Processed_Maze;

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

pub fn create_wall_nodes(image: &self::image::DynamicImage) -> Processed_Maze {
    let mut info = analyze_maze(image);
    let mut nodes: Vec<Node> = Vec::new();
    let (width, height) = image.dimensions();
    let mut id: u32 = 0;
    let mut x = 0;
    let mut y = 0;
    let node_length: u8;
    if info.wall_length % 2 == 0 {
        x = 1;
        y = 1;
    }
    node_length = info.wall_length + info.path_length;
    let xbounds = width - node_length as u32; 
    let ybounds = height - node_length as u32;
    while x <= xbounds {
        while y <= ybounds {
            let top_test = image.get_pixel(x + info.path_length as u32 - 1, y).data;
            let bottom_test = image.get_pixel(x + info.path_length as u32 - 1, y + info.path_length as u32 + 1).data;
            let left_test = image.get_pixel(x, y + info.path_length as u32 - 1).data;
            let right_test = image.get_pixel(x + info.path_length as u32 + 1, y + info.path_length as u32 - 1).data;

            let n = Node::new(id, x, y, left_test[0], right_test[0], bottom_test[0], top_test[0]);
            nodes.push(n);
            id += 1;
            y += node_length as u32;
        }
        x += node_length as u32;
        y = 1;
    }
    println!("{} Nodes created", nodes.len());
    
    println!("path length: {}", info.path_length);
    println!("wall length: {}", info.wall_length);
    
    // find entrance and exit node ids
    let maze_entrance = convert_xy_to_vecpos(&info.maze_openings[0], width, node_length);
    let maze_exit = convert_xy_to_vecpos(&info.maze_openings[info.path_length as usize], width, node_length);
    println!("pint1: ({},{})", &info.maze_openings[0].x, &info.maze_openings[0].y);
    println!("point2: ({},{})", &info.maze_openings[info.path_length as usize].x, &info.maze_openings[info.path_length as usize].y);
    
    println!("maze entrance id: {}", maze_entrance);
    println!("maze exit id: {}", maze_exit);

    Processed_Maze { starting_node: maze_entrance, ending_node: maze_exit, maze_nodes: nodes }
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

    let (path_length, maze_openings) = check_for_openings(&image, 0, true, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(Point{x: p.x, y: p.y});
        }
    }
    let (path_length, maze_openings) = check_for_openings(&image, img_width - 1, true, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(Point{x: p.x, y: p.y});
        }
    }

    let (path_length, maze_openings) = check_for_openings(&image, 1, false, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(Point{x: p.x, y: p.y});
        }
    }
    let (path_length, maze_openings) = check_for_openings(&image, img_height - 1, false, wall_color);
    if path_length != 0 {
        let (new_path, is_path_uniform) = check_path_length(path_width, path_length);
        if new_path != 0 {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(Point{x: p.x, y: p.y});
        }
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

fn convert_xy_to_vecpos(point: &Point, maze_width: u32, node_width: u8) -> u32 {
    let nodes_per_row = maze_width / node_width as u32;
    let x_pos = (point.x) / node_width as u32;
    let y_pos = (point.y) / node_width as u32;
    y_pos * nodes_per_row + x_pos
}

fn check_for_openings(image: &self::image::DynamicImage, wall_pos: u32, is_vertical: bool, wall_color: [u8; 4]) -> (u8, Vec<Point>) {
    let (img_width, img_height) = image.dimensions();
    let mut points: Vec<Point> = Vec::new();
    let mut path_length: u8 = 0;
    if is_vertical {
        for y in 1..img_height {
            let pixle_color = image.get_pixel(wall_pos, y).data;            
            if pixle_color != wall_color {            
                points.push(Point{x: wall_pos, y: y});
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
                points.push(Point{x: x, y: wall_pos});
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

