use minifb::{Key, Window, WindowOptions, Scale};

use crate::bitmap::{self, WIDTH, HEIGHT};
use crate::formula::{Tuppers, Constant};

const DISPLAY_TITLE: &str = "Tupperplot";
const DISPLAY_WIDTH: usize = WIDTH as usize;
const DISPLAY_HEIGHT: usize = HEIGHT as usize;

/// get em rgb value
#[inline]
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

// why do this when we can do the same at bitmap::plot_bitmap()
// during the same time as plotting just by filling a 2D buffer then flatten it! profit!
fn _top_down_plotter_linear(k: &Constant, buffer: &mut Vec<u32>) {
    // k constant
    let k = Tuppers::bignumber_k(k);

    let mut row = 0;

    // start from the left (vertical flip)
    let mut column = WIDTH;

    for (pointer, pixel) in buffer.iter_mut().enumerate() {
        // left <- right
        if pointer % DISPLAY_WIDTH == 0 {
            column -= 1;
            row = 0;
        }

        let tuppers = Tuppers {
            k: k.to_owned(),
            x: row,
            y: column
        };

        if tuppers.self_referential_formula_inverse() {
            *pixel = from_u8_rgb(255, 255, 255);
        }

        row += 1;
    }    
}

/// Draw the plot on a framebuffer with scaling.
pub fn draw_display_framebuffer(k: &Constant) {
    // get the flattened framebuffer from the plotted bitmap
    let buffer = bitmap::plot_bitmap(k);

    let mut window_opts = WindowOptions::default();
    window_opts.scale = Scale::X8; // minifb is such a breeze! <3

    let mut window = Window::new(
        DISPLAY_TITLE,
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        window_opts,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // set frames per second
    window.limit_update_rate(Some(std::time::Duration::from_micros(420)));

    // we need some animation tho!
    // kinda pseudo tho cus originally we drawing top->bottom then flipping it.
    let mut anim_buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // animate plotting by copying plotted buffer to an empty buffer pixel by pixel
        for pointer in 0..(anim_buffer.len() - 1) {
            anim_buffer[pointer] = buffer[pointer];

            window
                .update_with_buffer(anim_buffer.as_ref(), DISPLAY_WIDTH, DISPLAY_HEIGHT)
                .unwrap();            
        }
    }

    // showcase reeling through all constants (comment above rendering to show)
    let mut anim_buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

    let reel = vec![Constant::Tuppers, Constant::Pacman, Constant::Euler];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for k in reel.iter() {
            let buffer = bitmap::plot_bitmap(k);

            // animate plotting by copying plotted buffer to an empty buffer pixel by pixel
            for pointer in 0..(anim_buffer.len() - 1) {
                anim_buffer[pointer] = buffer[pointer];
    
                window
                    .update_with_buffer(anim_buffer.as_ref(), DISPLAY_WIDTH, DISPLAY_HEIGHT)
                    .unwrap();            
            }
        }
    }
}