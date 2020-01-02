// Scene representation module

use crate::math::Vec3;
use crate::math::Vec2i;
use crate::geometry::Ray;
use crate::geometry::Intersection;

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

pub trait SceneObject {
    fn norm(&self, point: &Vec3) -> Vec3;
    fn intersection(&self, ray: &Ray) -> Intersection;
    fn material(&self) -> &Material;
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
    fd: f32,
    camera_coord: Vec3
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            fd: 1.0,
            camera_coord: Vec3::new(0.0, 0.0, 0.0)
        }
    }

    fn screen_sample_coord(&self, pixel: &Vec2i, size: &Vec2i) -> Vec3 {
        let centered_x = pixel.x as f32 - 0.5 * size.x as f32;
        let centered_y = pixel.y as f32 - 0.5 * size.y as f32;
        Vec3::new(centered_x / (size.x as f32), centered_y / (size.y as f32), self.fd)
    }

    pub fn generate_ray(&self, pixel: &Vec2i, screen_size: &Vec2i) -> Ray {
        let sample_coord = self.screen_sample_coord(pixel, &screen_size);
        Ray::new(self.camera_coord.clone(), (sample_coord - self.camera_coord).normalize())
    }
}
