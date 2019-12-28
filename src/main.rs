mod image;
mod scene;

use scene::Vec3;
use scene::Vec3u8;
use image::Scaling;

fn basic_scene() -> scene::Scene {
    scene::Scene::new(
        vec![
            Box::new(scene::Sphere::new(
                Vec3::new(-1.0, 0.0, 6.0), 1.0, 
                scene::Material::new(
                    Vec3::new(0.01, 0.0, 0.01),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    100.0, 0.1))),
            Box::new(scene::Sphere::new(
                Vec3::new(1.0, 0.0, 4.0), 0.8,
                scene::Material::new(
                    Vec3::new(0.01, 0.01, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0, 0.5))),
            Box::new(scene::Plane::new(
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                scene::Material::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0, 0.8)))
        ],
        vec![
            Box::new(scene::Light::new(Vec3::new(0.0, 0.0, 0.0)))
        ]
    )
}

fn raytrace(scene_instance: scene::Scene) {
    let camera = scene::Camera::new();
    let mut screen = image::Image::<Vec3>::new(512, 512);
    scene::Raytracer::<Vec3>::raytrace(&camera, &scene_instance, &mut screen);
    screen.scale(Vec3::new(0.0, 0.0, 0.0), Vec3::new(255.0, 255.0, 255.0));   
    screen.flipy();
    let screenshot = image::Image::<Vec3u8>::from(screen);   
    match screenshot.write_pbm("img.pbm")  { Ok(_) => {}, Err(_) => {} };
}

fn main() {
    raytrace(basic_scene());
}
