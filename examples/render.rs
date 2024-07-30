use std::io::BufWriter;

use env_logger::Env;
use ray_tracing::camera::Camera;
use ray_tracing::ray::World;
use ray_tracing::sphere::Sphere;

fn main() {
    // Logger
    let env = Env::default()
        .default_filter_or("trace")
        .default_write_style_or("always");
    env_logger::init_from_env(env);

    // World
    let mut world = World::new();
    world.add(Sphere::new((0., 0., -1.).into(), 0.5).unwrap());
    world.add(Sphere::new((0., -100.5, -1.).into(), 100.).unwrap());

    let file = BufWriter::new(
        std::fs::File::options()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open("render.ppm")
            .expect("Could not create or open file `render.ppm`"),
    );

    let cam = Camera::new(16. / 9., 400);

    cam.render(world, file)
}
