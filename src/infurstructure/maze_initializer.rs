extern crate image;

use std::fs::File;
use std::path::Path;

use modals::wall_node::node;
use modals::node_info::point;
use modals::node_info::maze_info;

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

pub fn create_wall_nodes(image: &self::image::DynamicImage) -> Vec<node> {
    let mut nodes: Vec<node> = Vec::new();
    let info = analyze_maze(image);    
    println!("path length: {}", info.path_length);
    println!("wall length: {}", info.wall_length);
    println!("Openings: ");
    for p in &info.maze_openings {
        println!("({},{})", p.x, p.y);
    }
    let (width, height) = image.dimensions();
    let mut id: u32 = 1;
    let mut x = 0;
    let mut y = 0;
    let mut node_length = 0;
    if info.wall_length % 2 == 0 {
        x = 1;
        y = 1;
    }
    node_length = info.wall_length + info.path_length;
    while x <= width - node_length as u32 {
        while y <= height - node_length as u32 {
            let top_test = image.get_pixel(x + info.path_length as u32 - 1, y).data;
            let bottom_test = image.get_pixel(x + info.path_length as u32 - 1, y + info.path_length as u32 + 1).data;
            let left_test = image.get_pixel(x, y + info.path_length as u32 - 1).data;
            let right_test = image.get_pixel(x + info.path_length as u32 + 1, y + info.path_length as u32 - 1).data;

            let mut n = node::new(id, x, y, left_test[0], right_test[0], bottom_test[0], top_test[0]);
            nodes.push(n);
            id += 1;
            y += node_length as u32;
        }
        x += node_length as u32;
        y = 1;
    }
    println!("{} Nodes created", nodes.len());
    nodes
}
// need to decide how much to analyze, i dont think going through the entire maze is necessary 
// maybe just look around all the edges for opening and closing and a few lines?
fn analyze_maze(image: &self::image::DynamicImage) -> maze_info { 
    println!("Maze loaded scanning maze now...");
    let (img_width, img_height) = image.dimensions();       
    let wall_color = image.get_pixel(0, 0).data;
    let mut path_width: u8 = 0;
    let mut path_color = [0, 0, 0, 0];
    let mut openings: Vec<point> = Vec::new();    
    let mut is_Path_Uniform = false;

    let (path_length, maze_openings) = check_for_openings(&image, 0, true, wall_color);
    if path_length != 0 {
        let (new_path, is_Path_Uniform) = check_path_length(path_width, path_length);
        if (new_path != 0) {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(point{x: p.x, y: p.y});
        }
    }
    let (path_length, maze_openings) = check_for_openings(&image, img_width - 1, true, wall_color);
    if path_length != 0 {
        let (new_path, is_Path_Uniform) = check_path_length(path_width, path_length);
        if (new_path != 0) {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(point{x: p.x, y: p.y});
        }
    }

    let (path_length, maze_openings) = check_for_openings(&image, 1, false, wall_color);
    if path_length != 0 {
        let (new_path, is_Path_Uniform) = check_path_length(path_width, path_length);
        if (new_path != 0) {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(point{x: p.x, y: p.y});
        }
    }
    let (path_length, maze_openings) = check_for_openings(&image, img_height - 1, false, wall_color);
    if path_length != 0 {
        let (new_path, is_Path_Uniform) = check_path_length(path_width, path_length);
        if (new_path != 0) {
            path_width = new_path;
        }
    }
    if maze_openings.len() != 0 {
        for p in &maze_openings {
            openings.push(point{x: p.x, y: p.y});
        }
    }
    let wall_length = check_for_wall_length(&image, wall_color);

    maze_info { path_length: path_width, wall_length: wall_length, maze_openings: openings}
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

fn check_for_openings(image: &self::image::DynamicImage, wall_pos: u32, is_vertical: bool, wall_color: [u8; 4]) -> (u8, Vec<point>) {
    let (img_width, img_height) = image.dimensions();
    let mut points: Vec<point> = Vec::new();
    let mut path_color = [0, 0, 0, 0];
    let mut path_length: u8 = 0;
    if is_vertical {
        for y in 1..img_height {
            let pixle_color = image.get_pixel(wall_pos, y).data;
            if pixle_color != wall_color {            
                points.push(point{x: wall_pos, y: y});
                path_color = pixle_color;
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
                points.push(point{x: x, y: wall_pos});
                path_color = pixle_color;
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

