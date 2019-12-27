// Scene representation module

use std::clone::Clone;
use std::f32;
use std::cmp::min;
use std::result::Result;
use std::fmt::Display;
use std::ops::AddAssign;
use std::marker::PhantomData;

use crate::image::Image;

pub struct Vec2i { x: i32, y: i32 }
impl Vec2i { pub fn new (x: i32, y: i32) -> Vec2i { Vec2i {x: x, y: y} } }

#[derive(Clone, PartialEq, Debug, Default, Copy)]
pub struct Vec3 { x: f32, y: f32, z: f32 }
impl Vec3 {
    pub fn new (x: f32, y: f32, z: f32) -> Vec3 { Vec3 {x: x, y: y, z: z} } 
    pub fn sub (&self, operand: &Vec3) -> Vec3 { Vec3::new(self.x - operand.x, self.y - operand.y, self.z - operand.z) }
    pub fn add (&self, operand: &Vec3) -> Vec3 { Vec3::new(self.x + operand.x, self.y + operand.y, self.z + operand.z) }
    pub fn mul (&self, operand: f32) -> Vec3 { Vec3::new(self.x * operand, self.y * operand, self.z * operand) }
    pub fn mul3 (&self, operand: &Vec3) -> Vec3 { Vec3::new(self.x * operand.x, self.y * operand.y, self.z * operand.z) }
    pub fn dot (&self, operand: &Vec3) -> f32 { self.x * operand.x + self.y * operand.y + self.z * operand.z }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(&other);
    }
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
    pub alpha: f32
}
impl Material {
    pub fn new (ka: Vec3, kd: Vec3, ks: Vec3, alpha: f32) -> Material {
        Material {ka: ka, kd: kd, ks: ks, alpha: alpha}
    }
}

pub trait SceneObject {
    fn norm(&self, point: &Vec3) -> Vec3;
    fn intersection(&self, ray: &Ray) -> Result<Vec3, bool>;
    fn material(&self) -> &Material;
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

pub trait LightingTarget<T> {
    fn evaluate_lighting(v: &Vec3, point: &Vec3, norm: &Vec3, mat: &Material, scene: &Scene) -> T;
}
impl LightingTarget<Vec3> for Vec3 {
    fn evaluate_lighting(v: &Vec3, point: &Vec3, norm: &Vec3, mat: &Material, scene: &Scene) -> Vec3 {
        let mut illum = Vec3::new(0.0, 0.0, 0.0);
        for light in scene.lights.iter() {
            let lm = light.pos.sub(&point);
            let rm = norm.mul(2.0 * lm.dot(&norm)).sub(&lm);
            let col = light.ia.mul3(&mat.ka).add(
                &mat.kd.mul(lm.dot(&norm)).mul3(&light.id)).add(
                &mat.ks.mul(rm.dot(&v)).mul3(&light.is));
            illum.add(&col);
        }
        illum
    }
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
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
    let result = sphere.intersection(&ray);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), Vec3::new(0.0, 0.0, -1.0));
}
impl SceneObject for Sphere {
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
    fn material(&self) -> &Material {
        &self.material
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

pub struct Raytracer<T> {
    phantom: PhantomData<T>
}

impl<T: Clone + Display + Copy + Default + AddAssign + LightingTarget<T>> Raytracer<T> {
    fn raytrace_object(scene_object: &SceneObject, scene: &Scene, ray: &Ray) -> Result<T, bool> {
        let point = scene_object.intersection(ray)?;
        let norm = scene_object.norm(&point);
        let illum = T::evaluate_lighting(&ray.dir, &point, &norm, &scene_object.material(), scene);
        Ok(illum)
    }

    fn raytrace_scene(scene: &Scene, ray: &Ray) -> T {
        let mut retval = T::default();
        for scene_object in scene.scene_objects.iter() {
            match Raytracer::<T>::raytrace_object(&**scene_object, scene, ray) {
                Err(_) => {},
                Ok(illum) => { retval += illum; }
            };
        }
        retval
    }

    pub fn raytrace(camera: &Camera, scene: &Scene, image: &mut Image<T>) {
        let screen_size = Vec2i::new(image.width as i32, image.height as i32);
        let camera_coord = Vec3::new(0.0, 0.0, 0.0);       
        for y in 0..image.height {
            for x in 0..image.width {
                let sample_coord = camera.screen_sample_coord(&Vec2i::new(x as i32, y as i32), &screen_size);
                let ray = Ray::new(camera_coord.clone(), sample_coord.sub(&camera_coord));
                let sample_value: T = Raytracer::<T>::raytrace_scene(&scene, &ray);
                image.set(x, y, sample_value);
            }
        }
    }
}
