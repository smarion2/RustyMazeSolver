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
    // let (width, height) = image.dimensions();
    // let mut id: u32 = 1;
    // let mut x = 1;
    // let mut y = 1;
    // while x <= width - 6 {
    //     while y <= height - 6 {
    //         let top_test = image.get_pixel(x + 3, y).data;
    //         let bottom_test = image.get_pixel(x + 3, y + 5).data;
    //         let left_test = image.get_pixel(x, y + 3).data;
    //         let right_test = image.get_pixel(x + 5, y + 3).data;

    //         let mut n = node::new(id, x, y, left_test[0], right_test[0], bottom_test[0], top_test[0]);
    //         nodes.push(n);
    //         id += 1;
    //         y += 6;
    //     }
    //     x += 6;
    //     y = 1;
    // }
    nodes
}
// need to decide how much to analyze, i dont think going through the entire maze is necessary 
// maybe just look around all the edges for opening and closing and a few lines?
fn analyze_maze(image: &self::image::DynamicImage) -> maze_info { 
    println!("Maze loaded scanning maze now...");
    let (img_width, img_height) = image.dimensions();   
    let mut node_height = 0;
    let mut node_width = 0;
    let mut wall_color = image.get_pixel(0, 0).data;
    let mut path_color = [0, 0, 0, 0];
    let mut openings: Vec<point> = Vec::new();    

    // get path color  

    println!("Checking left wall");
    let (path_length, wall_length, openings) = check_walls(&image, 0, true, wall_color);
    println!("Path length: {}", path_length);
    println!("Wall length: {}", wall_length);
    if openings.len() != 0 {
        for p in &openings {
            println!("Opening found at x: {}, y: {}", p.x, p.y);
        }
    }
    println!("Checking right wall");
    let (path_length, wall_length, openings) = check_walls(&image, img_width - 1, true, wall_color);
    println!("Path length: {}", path_length);
    println!("Wall length: {}", wall_length);
    if openings.len() != 0 {
        for p in &openings {
            println!("Opening found at x: {}, y: {}", p.x, p.y);
        }
    }
    println!("Checking top wall");
    let (path_length, wall_length, openings) = check_walls(&image, 1, false, wall_color);    
    println!("Path length: {}", path_length);
    println!("Wall length: {}", wall_length);
    if openings.len() != 0 {
        for p in &openings {
            println!("Opening found at x: {}, y: {}", p.x, p.y);
        }
    }
    println!("checking bottom wall");
    let (path_length, wall_length, openings) = check_walls(&image, img_height - 1, false, wall_color);
    println!("Path length: {}", path_length);
    println!("Wall length: {}", wall_length);
    if openings.len() != 0 {
        for p in &openings {
            println!("Opening found at x: {}, y: {}", p.x, p.y);
        }
    }
    // not real info just making sure the function works
    maze_info { height: 6, width: 6, path_color: [255, 255, 255], maze_openings: openings}
}

fn check_walls(image: &self::image::DynamicImage, wall_pos: u32, is_vertical: bool, wall_color: [u8; 4]) -> (u32, u32, Vec<point>) {
    let (img_width, img_height) = image.dimensions();
    let mut points: Vec<point> = Vec::new();
    let mut path_color = [0, 0, 0, 0];
    let mut wall_length: u32 = 0;
    let mut path_length: u32 = 0;
    println!("Checking walls now...");
    if is_vertical {
        for y in 1..img_height {
            let pixle_color = image.get_pixel(wall_pos, y).data;
            if pixle_color != wall_color {            
                points.push(point{x: wall_pos, y: y});
                path_color = pixle_color;
                let mut length: u32 = 0;
                while pixle_color == image.get_pixel(wall_pos, y + length).data {
                    //println!("Length: {}", length);
                    length += 1;
                }
                if path_length == 0 {
                   path_length = length;
                   //println!("Path Length set: {}", length);
                }
                let mut length: u32 = 0;
                //let mut pixle_color = [0, 0, 0, 0];
                let wall_point = &points[0];
                while wall_color == image.get_pixel(wall_point.x + length, wall_point.y - 1).data {
                    //println!("wall Length: {}", length);
                    length += 1;
                }                
                if wall_length == 0 {
                    //println!("wall Length set: {}", length);
                    wall_length = length;
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
                let mut length: u32 = 0;
                while pixle_color == image.get_pixel(x + length, wall_pos).data {
                    length += 1;
                }
                if path_length == 0 {
                    path_length = length;
                }
                let mut length: u32 = 0;
                let wall_point = &points[0];
                println!("wall_pointx: {} wall_pointy: {}", wall_point.x - 1, wall_point.y);
                while wall_color == image.get_pixel(wall_point.x - 1, wall_point.y + length).data {
                    println!("wall found length: {}", length);
                    length += 1;
                }
                if wall_length == 0 {
                   wall_length = length;
                }
            }
        }
    }
    (path_length, wall_length, points)
}

