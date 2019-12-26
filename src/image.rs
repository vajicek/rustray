// Module for representation of image and operations on images.

use std::fs;
use std::io;
use std::io::prelude::*;
use std::clone::Clone;
use std::fmt::Display;
use std::convert::From;

use crate::scene::Vec3;

pub struct Image<T> {
    pub width: usize,
    pub height: usize,
    pixels: Vec<T>
}

pub trait Scaling<T> {
    fn scale(&mut self, from: T, to: T);
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
}

impl<f32> Scaling<f32> for Image<f32> {
    fn scale(&mut self, from: f32, to: f32) {
        let minValue = self.pixels.iter().fold(std::f32::MAX, |a, &b| a.min(b));
        let maxValue = self.pixels.iter().fold(std::f32::MIN, |a, &b| a.max(b));
        let range = (maxValue - minValue);
        let target_range = to - from;
        for pixel in &mut self.pixels {
            *pixel = from + target_range * (*pixel - minValue) / range;
        } 
    }
}

impl<Vec3> Scaling<Vec3> for Image<Vec3> {
    fn scale(&mut self, from: Vec3, to: Vec3) {
        /*
        let minValue = self.pixels.iter().fold(std::f32::MAX, |a, &b| a.min(b));
        let maxValue = self.pixels.iter().fold(std::f32::MIN, |a, &b| a.max(b));
        let range = (maxValue - minValue);
        let target_range = to - from;
        for pixel in &mut self.pixels {
            *pixel = from + target_range * (*pixel - minValue) / range;
        }
        */
    }
}

impl From<Image<f32>> for Image<u8> {
    fn from(w: Image<f32>) -> Image<u8> {
        let mut new_image = Image::<u8>::new(w.width, w.height);
        for y in 0..w.height {
            for x in 0..w.width {
                new_image.set(x, y, w.get(x, y) as u8);
            }
        }
        new_image
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