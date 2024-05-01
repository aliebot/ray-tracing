use std::io::{self, Write};

fn main() {
    // image
    let image_width = 256;
    let image_height = 256;

    // render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        io::stdout().flush().unwrap();
        for j in 0..image_width {
            let r = f64::from(j) / f64::from(image_width - 1);
            let g = f64::from(i) / f64::from(image_height - 1);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    print!("\rDone.                 \n");
}
