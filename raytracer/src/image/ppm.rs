use std::{fs::File, io, io::Write};

use crate::image::Color;

pub fn write_header(width: u16, height: u16, file: &mut File) -> io::Result<()> {
    write!(file, "P3\n{} {}\n255\n", width, height)?;
    Ok(())
}

pub fn write_color(color: &Color<u8>, file: &mut File) -> io::Result<()> {
    write!(file, "{} {} {}\n", color.r, color.g, color.b)?;
    Ok(())
}
