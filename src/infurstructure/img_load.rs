extern crate image;

use std::fs::File;
use std::path::Path;

use modals::wall_node;
use self::image::GenericImage;

pub fn open(file: String) {
    // Use the open function to load an image from a PAth.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", im.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    let fout = &mut File::create(&Path::new(&format!("{}.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    im.save(fout, image::PNG).unwrap();
    let mut test = String::new();
    let mut test1 = String::new();
    let mut test2 = String::new();
    let mut test3 = String::new();

    let n = wall_node::node::new(1, 1, 1, 1, 1, 1, test, test1, test2, test3);
}