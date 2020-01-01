// Raytracing and lighting model

use std::clone::Clone;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use crate::image::Image;
use crate::math::Vec3;
use crate::math::Vec2i;
use crate::scene::SceneObject;
use crate::scene::Material;
use crate::scene::Scene;
use crate::scene::Camera;
use crate::geometry::Ray;
use crate::geometry::Intersection;

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
            let mut gterm = 1.0;

            let ray = Ray::new(point, light.pos - point);
            let intersections : Vec<Intersection> = scene.scene_objects
                .iter()
                .map(|scene_object| scene_object.intersection(ray))
                .filter(|intersection_object| intersection_object.0.success);

            if !intersections.is_empty() {
                gterm = 0.0;
            }

                //TODO(vajicek): add geometry term for shadows
            illum += light.ia.mul3(&mat.ka) + col.mul(light_att * gterm);
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
        let intersections : Vec<(Intersection, &dyn SceneObject)> = scene.scene_objects
            .iter()
            .map(|scene_object| (scene_object.intersection(ray), &**scene_object))
            .filter(|intersection_object| intersection_object.0.success)
            .collect();
        if !intersections.is_empty() {
            let closest_intersection = intersections.iter().fold(intersections[0], |a, &b| if a.0.distance < b.0.distance { a } else { b });
            T::evaluate_lighting(
                &ray.dir.normalize(),
                &closest_intersection.0.position,
                &closest_intersection.0.norm,
                &closest_intersection.1.material(),
                scene,
                depth)
        } else {
            T::default()
        }
    }

    pub fn raytrace(camera: &Camera, scene: &Scene, image: &mut Image<T>) {
        let screen_size = Vec2i::new(image.width as i32, image.height as i32);
        for y in 0..image.height {
            for x in 0..image.width {
                let ray = camera.generate_ray(&Vec2i::new(x as i32, y as i32), &screen_size);
                let sample_value: T = Raytracer::<T>::raytrace_scene(&scene, &ray, 1);
                image.set(x, y, sample_value);
            }
        }
    }
}
