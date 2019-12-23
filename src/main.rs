mod image;
mod scene;

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
    //let cube = scene::Cube::new();
    //let mut scene = scene::Scene::new(vec![Box::new(cube)]);
    let mut scene = scene::Scene::new(vec![
        Box::new(scene::Sphere::new(scene::Vec3::new(-1.0, 1.0, 5.0), 1.0)),
        Box::new(scene::Sphere::new(scene::Vec3::new(1.0, 0.0, 3.0), 1.0))
    ]);

    let camera = scene::Camera::new();
    let mut screen = image::Image::<f32>::new(256, 256);
    camera.raytrace(scene, &mut screen);

    let image_u8 = screen.convert::<image::Bip>();

    /*
    match screen.write_pbm("img.pbm")  {
        Ok(_) => {},
        Err(_) => {},
    };
    */
}

fn main() {
    create_and_save_image();
    raytrace();
}
