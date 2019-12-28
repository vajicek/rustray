// Scene representation module

use std::clone::Clone;
use std::f32;
use std::cmp::min;
use std::result::Result;
use std::fmt::Display;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::Add;
use std::marker::PhantomData;

use crate::image::Image;

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
    pub fn min_element(&self) -> f32 { self.x.min(self.y).min(self.z) }
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

pub struct Light {
    pub ia: Vec3,
    pub id: Vec3,
    pub is: Vec3,
    pub pos: Vec3,
    pub dir: Vec3,
    pub att: Vec3
}
impl Light {
    pub fn new (pos: Vec3) -> Light {
        Light {
            ia: Vec3::new(0.01, 0.01, 0.01),
            id: Vec3::new(0.01, 0.01, 0.01),
            is: Vec3::new(0.01, 0.01, 0.01),
            pos: pos,
            dir: Vec3::new(0.0, 0.0, 0.0),
            att: Vec3::new(1.0, 1.0, 1.0)
        }
    } 
} 

pub struct Material {
    pub ka: Vec3,
    pub kd: Vec3,
    pub ks: Vec3,
    pub alpha: f32,
    pub reflection: f32
}
impl Material {
    pub fn new (ka: Vec3, kd: Vec3, ks: Vec3, alpha: f32, reflection: f32) -> Material {
        Material {ka: ka, kd: kd, ks: ks, alpha: alpha, reflection: reflection}
    }
}

struct Ray {
    from: Vec3,
    dir: Vec3
}
impl Ray {
    pub fn new (from: Vec3, dir: Vec3) -> Ray {
        Ray {from: from, dir: dir}
    }
}

#[derive(Copy, Clone)]
struct Intersection {
    position: Vec3,
    norm: Vec3,
    distance: f32,
    success: bool
}
impl Intersection {
    pub fn new(position: Vec3, norm: Vec3, distance: f32) -> Intersection { Intersection { position: position, norm: norm, distance: distance, success: true } }
    pub fn no_intersection() -> Intersection { Intersection { position: Vec3::default(), norm: Vec3::default(), distance: 0.0, success: false } }
}

pub trait SceneObject {
    fn norm(&self, point: &Vec3) -> Vec3;
    fn intersection(&self, ray: &Ray) -> Intersection;
    fn material(&self) -> &Material;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere { Sphere {center: center, radius: radius, material: material} }
}
#[test]
fn test_intersection() {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0,
        Material::new(Vec3::new(0.1, 0.1, 0.1), Vec3::new(0.1, 0.1, 0.1), Vec3::new(0.0, 0.0, 0.0), 1.0));
    let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
    let result = sphere.intersection(&ray);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), Vec3::new(0.0, 0.0, -1.0));
}
impl SceneObject for Sphere {
    fn material(&self) -> &Material { &self.material }
    fn norm(&self, point: &Vec3) -> Vec3 { (*point - self.center).normalize() }
    fn intersection(&self, ray: &Ray) -> Intersection {
        let f_x = ray.from - self.center;
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * f_x.dot(&ray.dir);
        let c = f_x.dot(&f_x) - self.radius * self.radius; 

        let discriminant = b * b - 4.0 * a * c;
        let mut t = 0.0;

        if discriminant < 0.0 {
            return Intersection::no_intersection();
        } else if discriminant < 1e-5 {
            t = -b / (2.0 * a);
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            t = t1.min(t2);
        }
        let point = ray.from + ray.dir.mul(t);
        Intersection::new(point, self.norm(&point), t)
    }
}

pub struct Plane {
    origin: Vec3,
    norm: Vec3,
    material: Material
}
impl Plane {
    pub fn new(origin: Vec3, norm: Vec3, material: Material) -> Plane { Plane {origin: origin, norm: norm.normalize(), material: material} }
}
impl SceneObject for Plane {
    fn material(&self) -> &Material { &self.material }
    fn norm(&self, point: &Vec3) -> Vec3 { self.norm }
    fn intersection(&self, ray: &Ray) -> Intersection {
        let d = self.norm.dot(&ray.dir);
        let mut t = 0.0;
        if d.abs() < 1e-5 {
            return Intersection::no_intersection();
        } else {
            t = (self.norm.dot(&self.origin) - self.norm.dot(&ray.from)) / d;
            if t <= 1e-5 {
                return Intersection::no_intersection();
            } else {
                let point = ray.from + ray.dir.mul(t);
                Intersection::new(point, self.norm(&point), t)
            }
        }
    }
}

pub struct Scene {
    pub scene_objects: Vec<Box<dyn SceneObject>>,
    pub lights: Vec<Box<Light>>
}
impl Scene {
    pub fn new(scene_objects: Vec<Box<dyn SceneObject>>, lights: Vec<Box<Light>>) -> Scene {
        Scene {
            scene_objects: scene_objects,
            lights: lights
        }
    }
}

pub struct Camera {
    fd: f32
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            fd: 1.0
        }
    }
    pub fn screen_sample_coord(&self, pixel: &Vec2i, size: &Vec2i) -> Vec3 {
        let centered_x = pixel.x as f32 - 0.5 * size.x as f32;
        let centered_y = pixel.y as f32 - 0.5 * size.y as f32;
        Vec3::new(centered_x / (size.x as f32), centered_y / (size.y as f32), self.fd)
    }
}

pub trait LightingTarget<T> {
    fn evaluate_lighting(v: &Vec3, point: &Vec3, norm: &Vec3, mat: &Material, scene: &Scene, depth: i32) -> T;
}
impl LightingTarget<Vec3> for Vec3 {
    fn evaluate_lighting(viewer_dir: &Vec3, point: &Vec3, norm: &Vec3, mat: &Material, scene: &Scene, depth: i32) -> Vec3 {
        let mut illum = Vec3::new(0.0, 0.0, 0.0);
        for light in scene.lights.iter() {
            let to_light = light.pos - *point;
            let light_dist = to_light.len();
            let to_light_dir = to_light.mul(1.0 / light_dist);
            let light_att = 1.0 / light.att.dot(&Vec3::new(1.0, light_dist, light_dist * light_dist));
            let reflected_dir = to_light_dir.reflect(norm);
            let col = mat.kd.mul(to_light_dir.dot(&norm).max(0.0)).mul3(&light.id) +
                mat.ks.mul((-reflected_dir.dot(&viewer_dir)).max(0.0).powf(mat.alpha)).mul3(&light.is);
            illum += light.ia.mul3(&mat.ka) + col.mul(light_att);
        }

        if depth > 0 {
            let ray = Ray::new(*point, viewer_dir.reflect(norm));
            illum = illum.mul(1.0 - mat.reflection) +
                Raytracer::<Vec3>::raytrace_scene(scene, &ray, depth - 1).mul(mat.reflection);    
        }

        illum
    }
}

pub struct Raytracer<T> {
    phantom: PhantomData<T>
}
impl<T: Clone + Display + Copy + Default + AddAssign + LightingTarget<T>> Raytracer<T> {

    fn raytrace_scene(scene: &Scene, ray: &Ray, depth: i32) -> T {
        let mut retval = T::default();

        let intersections: Vec<(Intersection, &SceneObject)> = scene.scene_objects
            .iter()
            .map(|scene_object| (scene_object.intersection(ray), &**scene_object))
            .filter(|intersection_object| intersection_object.0.success)
            .collect();

        if !intersections.is_empty() {
            let closest_intersection = intersections.iter().fold(intersections[0], |a, &b| if a.0.distance < b.0.distance { a } else { b });
            retval = T::evaluate_lighting(
                &ray.dir.normalize(),
                &closest_intersection.0.position,
                &closest_intersection.0.norm,
                &closest_intersection.1.material(),
                scene,
                depth);
        }
        retval
    }

    pub fn raytrace(camera: &Camera, scene: &Scene, image: &mut Image<T>) {
        let screen_size = Vec2i::new(image.width as i32, image.height as i32);
        let camera_coord = Vec3::new(0.0, 0.0, 0.0);       
        for y in 0..image.height {
            for x in 0..image.width {
                let sample_coord = camera.screen_sample_coord(&Vec2i::new(x as i32, y as i32), &screen_size);
                let ray = Ray::new(camera_coord.clone(), (sample_coord - camera_coord).normalize());
                let sample_value: T = Raytracer::<T>::raytrace_scene(&scene, &ray, 1);
                image.set(x, y, sample_value);
            }
        }
    }
}
