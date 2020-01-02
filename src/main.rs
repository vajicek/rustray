mod geometry;
mod image;
mod math;
mod raytracing;
mod scene;

use geometry::Plane;
use geometry::Sphere;
use image::Image;
use image::Scaling;
use math::Vec3;
use math::Vec3u8;
use raytracing::Raytracer;
use scene::Camera;
use scene::Light;
use scene::Material;
use scene::Scene;

fn scene(light_pos: Vec3) -> scene::Scene {
    Scene::new(
        vec![
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, 6.0), 1.0,
                Material::new(
                    Vec3::new(0.01, 0.0, 0.01),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    100.0, 0.1))),
            Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, 4.0), 0.8,
                Material::new(
                    Vec3::new(0.01, 0.01, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0, 0.5))),
            Box::new(Plane::new(
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Material::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0, 0.8)))
        ],
        vec![
            Box::new(Light::new(light_pos))
        ]
    )
}

fn basic_scene() -> scene::Scene {
    scene(Vec3::new(0.0, 5.0, 4.0))
}

fn raytrace(scene_instance: scene::Scene, filename: String) {
    let camera = Camera::new();
    let mut screen = Image::<Vec3>::new(512, 512);
    Raytracer::<Vec3>::raytrace(&camera, &scene_instance, &mut screen);
    screen.scale(Vec3::new(0.0, 0.0, 0.0), Vec3::new(255.0, 255.0, 255.0));
    screen.flipy();
    let screenshot = Image::<Vec3u8>::from(screen);
    match screenshot.write_pbm(filename)  { Ok(_) => {}, Err(_) => {} };
}

fn rayrace_sequence(images: i32) {
    for i in 0..images {
        let a = std::f32::consts::PI * 2.0 / 360.0 * (i as f32) * 5.0;
        let pos = Vec3::new(5.0 * a.cos(), 5.0, 5.0 * a.sin());
        raytrace(scene(pos), format!("output/img{:0>4}.pbm", i));
    }
}

fn main() {
    //rayrace_sequence(2 * 72);
    raytrace(basic_scene(), "output/img.pbm".to_string());
}