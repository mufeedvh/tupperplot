use bmp::Image;
use rustbitmap::BitMap;

use crate::formula::{Constant, Tuppers};
use crate::display::from_u8_rgb;

pub const WIDTH: u32 = 106;
pub const HEIGHT: u32 = 17;

/// Plot the bitmap
pub fn plot_bitmap(k: &Constant) -> Vec<u32> {
    let mut image = Image::new(WIDTH, HEIGHT);

    let mut buffer: Vec<Vec<u32>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];

    // k constant
    let k = Tuppers::bignumber_k(k);

    // walk through bitmap tiles
    for (x, y) in image.coordinates() {
        let tuppers = Tuppers {
            k: k.to_owned(),
            x,
            y
        };

        // should i plot this pixel or not?
        if tuppers.self_referential_formula_inverse() {
            // y -> to flip vertically
            image.set_pixel(x, HEIGHT - y - 1, bmp::consts::WHITE);
            buffer[(HEIGHT - y - 1) as usize][x as usize] = from_u8_rgb(255, 255, 255);
        }
    }

    image.save("output.bmp").unwrap();

    // flatten the 2D Vector to a single plane
    buffer.into_iter().flatten().collect()
}

/// Resize the bitmap image by a factor of 100.0 (bitmap bilinear interpolation)
pub fn resize_bitmap() {
    let mut bitmap = BitMap::read("output.bmp").unwrap();
    bitmap.resize_by(100.0).unwrap();
    bitmap.save_as("scaled.bmp").unwrap();    
}