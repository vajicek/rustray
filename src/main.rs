mod image;
mod scene;

use scene::Vec3;

//TODO(vajicek): remove
fn create_and_save_image() {
    let mut im = image::Image::<u8>::new(256, 256);
    im.dump_info();
    im.checkerboard(32, 0, 255);
    match im.write_pbm("img.pbm")  {
        Ok(_) => {},
        Err(_) => {},
    };
}

fn raytrace() {
    let mut scene_instance = scene::Scene::new(
        vec![
            Box::new(scene::Sphere::new(
                Vec3::new(-1.0, 0.0, 5.0), 1.0, 
                scene::Material::new(
                    Vec3::new(0.1, 0.1, 0.1),
                    Vec3::new(0.1, 0.1, 0.1),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0))),
            Box::new(scene::Sphere::new(
                Vec3::new(1.0, 0.0, 4.0), 1.0,
                scene::Material::new(
                    Vec3::new(0.1, 0.1, 0.1),
                    Vec3::new(0.1, 0.1, 0.1),
                    Vec3::new(0.0, 0.0, 0.0),
                    1.0)))
        ],
        vec![
            Box::new(scene::Light::new(Vec3::new(0.0, 0.0, 0.0)))
        ]
    );

    let camera = scene::Camera::new();
    let mut screen = image::Image::<Vec3>::new(256, 256);

    scene::raytrace(&camera, &scene_instance, &mut screen);

    screen.scale(Vec3::new(0.0, 0.0, 0.0), Vec3::new(255.0, 255.0, 255.0));   
    /*let screenshot = image::Image::<u8>::from(screen);

    match screenshot.write_pbm("img.pbm")  {
        Ok(_) => {},
        Err(_) => {},
    };*/
}

fn main() {
    create_and_save_image();
    raytrace();
}
