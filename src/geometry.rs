// Geometry primitives and scene geometry

use std::clone::Clone;

use crate::math::Vec3;
use crate::scene::Material;
use crate::scene::SceneObject;

pub struct Ray {
    pub from: Vec3,
    pub dir: Vec3
}
impl Ray {
    pub fn new (from: Vec3, dir: Vec3) -> Ray { Ray {from: from, dir: dir} }
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub position: Vec3,
    pub norm: Vec3,
    pub distance: f32,
    pub success: bool
}
impl Intersection {
    pub fn new(position: Vec3, norm: Vec3, distance: f32) -> Intersection { Intersection { position: position, norm: norm, distance: distance, success: true } }
    pub fn no_intersection() -> Intersection { Intersection { position: Vec3::default(), norm: Vec3::default(), distance: 0.0, success: false } }
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
        let t;

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
    fn norm(&self, _point: &Vec3) -> Vec3 { self.norm }
    fn intersection(&self, ray: &Ray) -> Intersection {
        let d = self.norm.dot(&ray.dir);
        if d.abs() < 1e-5 {
            return Intersection::no_intersection();
        } else {
            let t = (self.norm.dot(&self.origin) - self.norm.dot(&ray.from)) / d;
            if t <= 1e-5 {
                return Intersection::no_intersection();
            } else {
                let point = ray.from + ray.dir.mul(t);
                Intersection::new(point, self.norm(&point), t)
            }
        }
    }
}
