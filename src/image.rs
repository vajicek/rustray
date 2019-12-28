// Module for representation of image and operations on images.

use std::fs;
use std::io;
use std::io::prelude::*;
use std::clone::Clone;
use std::fmt::Display;
use std::convert::From;
use std::ops::Sub;
use std::ops::Add;

use crate::scene::Vec3;
use crate::scene::Vec3u8;
use crate::scene::Bounds;
use crate::scene::Comparable;

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

    pub fn flipy(&mut self) {
        for y in 0..(self.height / 2) {
            for x in 0..self.width {
                let v1 = self.get(x, y);
                let v2 = self.get(x, self.height - y - 1);
                self.set(x, y, v2);
                self.set(x, self.height - y - 1, v1);
            }
        }
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

impl Scaling<f32> for Image<f32> {
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

impl Scaling<Vec3> for Image<Vec3> {
    fn scale(&mut self, from: Vec3, to: Vec3) {
        let min_value = self.pixels.iter().fold(Vec3::max_value(), |a, &b| a.min(&b));
        let max_value = self.pixels.iter().fold(Vec3::min_value(), |a, &b| a.max(&b));
        let range1 = (max_value - min_value).max_element();
        let target_range1 = (to - from).max_element();
        for pixel in &mut self.pixels {
            *pixel = from + (*pixel - min_value).mul(target_range1 / range1);
        }
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

impl From<Image<Vec3>> for Image<Vec3u8> {
    fn from(w: Image<Vec3>) -> Image<Vec3u8> {
        let mut new_image = Image::<Vec3u8>::new(w.width, w.height);
        for y in 0..w.height {
            for x in 0..w.width {
                let element = w.get(x, y);
                new_image.set(x, y, Vec3u8::new(element.x as u8, element.y as u8, element.z as u8));
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

impl Image<Vec3u8> {
    //TODO(vajicek): separate writer module
    pub fn write_pbm(&self, filename: &str) -> std::io::Result<()> {
        let file = fs::File::create(filename)?;
        let mut writer = io::BufWriter::new(&file);
        writer.write_fmt(format_args!("P3\n"))?;
        writer.write_fmt(format_args!("{} {}\n", self.width, self.height))?;
        writer.write_fmt(format_args!("255\n"))?;
        for y in 0..self.height {
            for x in 0..self.width {
                let element = self.get(x, y);
                writer.write_fmt(format_args!("{} {} {}\n", element.x, element.y, element.z))?;
            }
        }
        Ok(())
    }
}