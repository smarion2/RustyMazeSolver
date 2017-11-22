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

    let fout = &mut File::create(&Path::new(&format!("{}.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    //im.save(fout, image::PNG).unwrap();
    
    return im;
}

fn create_wall_nodes(image: self::image::DynamicImage) -> Vec<node> {
    let mut nodes: Vec<node> = Vec::new();
    let mut test = String::new();
    let mut test1 = String::new();
    let mut test2 = String::new();
    let mut test3 = String::new();

    let mut n = node::new(1, 1, 1, 1, 1, 1, test, test1, test2, test3);
    println!("Node crate with id: {}", n.node_id);
    nodes.push(n);

    nodes
}