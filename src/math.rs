// Vector math

use std::clone::Clone;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

pub trait Bounds {
    fn min_value() -> Self;
    fn max_value() -> Self;
}
pub trait Comparable {
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
}

pub struct Vec2i { pub x: i32, pub y: i32 }
impl Vec2i { pub fn new (x: i32, y: i32) -> Vec2i { Vec2i {x: x, y: y} } }

#[derive(Clone, Debug, Default, Copy, PartialEq)]
pub struct Vec3u8 { pub x: u8, pub y: u8, pub z: u8 }
impl Vec3u8 { pub fn new (x: u8, y: u8, z: u8) -> Vec3u8 { Vec3u8 {x: x, y: y, z: z} } }
impl Display for Vec3u8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug, Default, Copy, PartialEq)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }
impl Vec3 {
    pub fn new (x: f32, y: f32, z: f32) -> Vec3 { Vec3 {x: x, y: y, z: z} }
    pub fn mul (&self, operand: f32) -> Vec3 { Vec3::new(self.x * operand, self.y * operand, self.z * operand) }
    pub fn mul3 (&self, operand: &Vec3) -> Vec3 { Vec3::new(self.x * operand.x, self.y * operand.y, self.z * operand.z) }
    pub fn dot (&self, operand: &Vec3) -> f32 { self.x * operand.x + self.y * operand.y + self.z * operand.z }
    pub fn max_element(&self) -> f32 { self.x.max(self.y).max(self.z) }
    pub fn normalize(&self) -> Vec3 { self.mul(1.0 / self.len()) }
    pub fn len(&self) -> f32 { self.dot(self).sqrt() }
    pub fn reflect(&self, norm: &Vec3) -> Vec3 { norm.mul(2.0 * self.dot(norm)) - *self }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) { *self = *self + other; }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self { Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) }
}
impl Bounds for Vec3 {
    fn min_value() -> Vec3 { Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN) }
    fn max_value() -> Vec3 { Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX) }
}
impl Comparable for Vec3 {
    fn min(&self, other: &Vec3) -> Vec3 { if self.max_element() < other.max_element() { *self } else { *other } }
    fn max(&self, other: &Vec3) -> Vec3 { if self.max_element() > other.max_element() { *self } else { *other } }
}
#[test]
fn test_ord() {
    let v1 = Vec3::new(1.0, 0.0, 2.0);
    let v2 = Vec3::new(2.0, 0.0, 4.0);
    let v3 = Vec3::new(1.0, 0.0, 1.0);

    assert_eq!(v1.min(&v2), v1);
    assert_eq!(v1.max(&v2), v2);

    let vec3_vector = vec![v1, v2, v3];
    let min_value = vec3_vector.iter().fold(Vec3::max_value(), |a, &b| a.min(&b));
    assert_eq!(min_value, v3);
    let max_value = vec3_vector.iter().fold(Vec3::min_value(), |a, &b| a.max(&b));
    assert_eq!(max_value, v2);
}