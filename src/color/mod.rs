use crate::vec3::Vec3;
use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color(out: &mut dyn Write, pixel_color: Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // convert [0,1] -> [0, 255]
    let r_byte = (255.999 * r) as u8;
    let g_byte = (255.999 * g) as u8;
    let b_byte = (255.999 * b) as u8;

    write!(out, "{} {} {}\n", r_byte, g_byte, b_byte);
}
