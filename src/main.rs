mod formula;
mod bitmap;
mod display;
use formula::Constant;

fn main() {
    // get cli args
    let args: Vec<String> = std::env::args().collect();

    let mut k: Constant = Constant::Tuppers;

    // plot it!
    if args.len() > 1 {
        let value = &args[1];

        match value.as_str() {
            "tuppers" => {
                k = Constant::Tuppers;
                bitmap::plot_bitmap(&k);
            },
            "pacman" => {
                k = Constant::Pacman;
                bitmap::plot_bitmap(&k);
            },
            "euler" => {
                k = Constant::Euler;
                bitmap::plot_bitmap(&k);
            },
            _ => {
                k = Constant::Tuppers;
                bitmap::plot_bitmap(&k);
            }
        };
    } else {
        bitmap::plot_bitmap(&k);
    }

    // resize it?!
    bitmap::resize_bitmap();

    // now plot em on a screen!
    display::draw_display_framebuffer(&k);
}
