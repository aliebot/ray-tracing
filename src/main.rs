use std::io::{self, Write};

mod color;
use crate::color::*;
mod vec3;

fn main() {
    // image
    let image_width = 256;
    let image_height = 256;

    // render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - i);
        io::stdout().flush().unwrap();
        for j in 0..image_width {
            let pixel_color = Color::new(
                f64::from(j) / (image_width - 1),
                f64::from(i) / (image_height - 1),
                0.0,
            );

            write_color(&mut std::io::stdout(), pixel_color)
        }
    }
    print!("\rDone.                 \n");
}
