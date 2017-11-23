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

pub fn create_wall_nodes(image: self::image::DynamicImage) -> Vec<node> {
    let mut nodes: Vec<node> = Vec::new();
    
    let (width, height) = image.dimensions();
    let mut id: u32 = 1;
    let mut x = 1;
    let mut y = 1;
    while x <= width - 6 {
        while y <= height - 6 {
            let top_test = image.get_pixel(x + 3, y).data;
            let bottom_test = image.get_pixel(x + 3, y + 5).data;
            let left_test = image.get_pixel(x, y + 3).data;
            let right_test = image.get_pixel(x + 5, y + 3).data;

            let mut n = node::new(id, x, y, left_test[0], right_test[0], bottom_test[0], top_test[0]);
            nodes.push(n);
            id += 1;
            y += 6;
        }
        x += 6;
        y = 1;
    }
    nodes
}
// need to decide how much to analyze, i dont think going through the entire maze is necessary 
// maybe just look around all the edges for opening and closing and a few lines?
fn analyze_maze(image: self::image::DynamicImage) -> maze_info {
    let (img_width, img_height) = image.dimensions();
    let mut node_height = 0;
    let mut node_width = 0;
    let mut wall_color = [0, 0, 0, 0];
    let mut path_color = [0, 0, 0, 0];
    let mut current_color = [0, 0, 0, 0];
    // get path color
    for y in img_height {
        for x in img_width {
            let pixle = image.get_pixel(x, y).data;
            if pixle != current_color {
                current_color = pixle;
                let mut length = 1;
                while current_color == image.get_pixel(x + length, y).data {
                    length += 1;
                }
            }


        }           
    }

    while x <= img_width {
        while y <= img_height {

        }
    }
    
    // not real info just making sure the function works
    maze_info { height: 6, width: 6, path_color: [255, 255, 255], maze_opening: point{x, y}, maze_ending: point{x, y} }
}

