// Module for representation of image and operations on images.

use std::fs;
use std::io;
use std::io::prelude::*;
use std::clone::Clone;
use std::fmt::Display;
use std::convert::From;

pub struct Image<T> {
    pub width: usize,
    pub height: usize,
    pixels: Vec<T>
}

impl<T: Clone + Display + Copy + Default> Image<T> {
    // Image constructor and memmory allocation
    pub fn new(width: usize, height: usize) -> Image<T> {
        Image {
            width: width,
            height: height,
            pixels: vec![T::default(); width * height]
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.pixels[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.pixels[y * self.width + x] = val;
    }

    // Debug info
    pub fn dump_info(&self) {
        println!("Image {} x {}", self.width, self.height)
    }

    // TODO(vajicek): separate module
    pub fn checkerboard(&mut self, size: usize, min: T, max: T) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x / size + y / size) % 2 == 0 {
                    self.set(x, y, min);
                } else {
                    self.set(x, y, max);
                }
            }
        }
    }

    pub fn convert<P: Clone + Display + Copy + Default + From<f32>>(&self) -> Image<P> {
        let new_image = Image::<P>::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let v = P::from(self.get(x, y));
                new_image.set(x, y, v);
            }
        }
        new_image
    }
}

struct Bip(u8 + );

impl From<f32> for Bip {
    fn from(value: f32) -> Self {
        Bip(value as u8)
    }
}


impl Image<u8> {
    //TODO(vajicek): separate writer module
    pub fn write_pbm(&self, filename: &str) -> std::io::Result<()> {
        let file = fs::File::create(filename)?;
        let mut writer = io::BufWriter::new(&file);
        writer.write_fmt(format_args!("P2\n"))?;
        writer.write_fmt(format_args!("{} {}\n", self.width, self.height))?;
        writer.write_fmt(format_args!("255\n"))?;
        for y in 0..self.height {
            for x in 0..self.width {
                writer.write_fmt(format_args!("{} ", self.get(x, y)))?;
            }
            writer.write_fmt(format_args!("\n"))?;
        }
        Ok(())
    }
}