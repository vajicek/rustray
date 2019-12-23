// Scene representation module

use std::clone::Clone;
use std::f32;
use std::cmp::min;
use std::result::Result;

use crate::image::Image;

pub trait SceneObject {
    fn raytrace(&self, ray: &Ray) -> Result<f32, bool>;
}

pub struct Vec2i { x: i32, y: i32 }
impl Vec2i { pub fn new (x: i32, y: i32) -> Vec2i { Vec2i {x: x, y: y} } }

#[derive(Clone, PartialEq, Debug)]
pub struct Vec3 { x: f32, y: f32, z: f32 }
impl Vec3 {
    pub fn new (x: f32, y: f32, z: f32) -> Vec3 { Vec3 {x: x, y: y, z: z} } 
    pub fn sub (&self, operand: &Vec3) -> Vec3 { Vec3::new(self.x - operand.x, self.y - operand.y, self.z - operand.z) }
    pub fn add (&self, operand: &Vec3) -> Vec3 { Vec3::new(self.x + operand.x, self.y + operand.y, self.z + operand.z) }
    pub fn mul (&self, operand: f32) -> Vec3 { Vec3::new(self.x * operand, self.y * operand, self.z * operand) }
    pub fn dot (&self, operand: &Vec3) -> f32 { self.x * operand.x + self.y * operand.y + self.z * operand.z }
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

pub struct Cube {
}
impl Cube {
    pub fn new() -> Cube { Cube {} }
}
impl SceneObject for Cube {
    fn raytrace(&self, ray: &Ray) -> Result<f32, bool> {
        Ok(0.0)
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere { Sphere {center: center, radius: radius} }
    fn grad(&self, point: &Vec3) -> Vec3 { point.sub(&self.center).mul(2.0) }
    fn norm(&self, point: &Vec3) -> Vec3 { point.sub(&self.center) }
    fn intersection(&self, ray: &Ray) -> Result<Vec3, bool> {
        let f_x = ray.from.sub(&self.center);
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * f_x.dot(&ray.dir);
        let c = f_x.dot(&f_x) - self.radius * self.radius; 

        let discriminant = b * b - 4.0 * a * c;
        let mut t = 0.0;

        if discriminant < 0.0 {
            return Err(false);
        } else if discriminant < 1e-5 {
            t = -b / (2.0 * a);
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            t = t1.min(t2);
        }
        let inter = ray.from.add(&ray.dir.mul(t));
        Ok(inter)
    }
}
#[test]
fn test_intersection() {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
    let result = sphere.intersection(&ray);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), Vec3::new(0.0, 0.0, -1.0));
}
impl SceneObject for Sphere {
    fn raytrace(&self, ray: &Ray) -> Result<f32, bool> {
        let point = self.intersection(ray)?;
        let norm = self.norm(&point);
        // model
        let illum = norm.dot(&ray.dir.mul(-1.0)).max(0.0);
        //let grad = self.grad(point);
        Ok(illum)
    }
}

pub struct Scene {
    pub scene_objects: Vec<Box<dyn SceneObject>>
}
impl Scene {
    pub fn new(scene_objects: Vec<Box<dyn SceneObject>>) -> Scene {
        Scene {
            scene_objects: scene_objects
        }
    }
    pub fn raytrace(&self, ray: &Ray) -> f32 {
        let mut retval: f32 = 0.0;
        for scene_object in self.scene_objects.iter() {
            retval += match scene_object.raytrace(&ray) {
                Err(_) => 0.0,
                Ok(illum) => illum * 255.0
            };
        }
        retval
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

    fn screen_sample(&self, pixel: &Vec2i, size: &Vec2i) -> Vec3 {
        let centered_x = pixel.x as f32 - 0.5 * size.x as f32;
        let centered_y = pixel.y as f32 - 0.5 * size.y as f32;
        Vec3::new(centered_x / (size.x as f32), centered_y / (size.y as f32), self.fd)
    }

    pub fn raytrace(&self, scene: Scene, image: &mut Image<f32>) {
        let screen_size = Vec2i::new(image.width as i32, image.height as i32);
        let camera_coord = Vec3::new(0.0, 0.0, 0.0);       
        for y in 0..image.height {
            for x in 0..image.width {
                let screen_coord = self.screen_sample(&Vec2i::new(x as i32, y as i32), &screen_size);
                let ray = Ray::new(camera_coord.clone(), screen_coord.sub(&camera_coord));
                image.set(x, y, scene.raytrace(&ray));
            }
        }
    }
}

