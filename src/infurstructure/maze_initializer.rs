extern crate image;

use std::fs::File;
use std::path::Path;

use modals::wall_node::node;
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