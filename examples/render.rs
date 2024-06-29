use std::io::{self, BufWriter, Write};

use env_logger::Env;
use ray_tracing::color::Color;
use ray_tracing::ray::{Hittable, Ray, World};
use ray_tracing::sphere::Sphere;
use ray_tracing::vec::Vec3;

fn write_ppm_color<W: Write>(mut write: W, color: Color) -> io::Result<()> {
    writeln!(write, "{} {} {}", color.r, color.g, color.b)
}

fn ray_color<H: Hittable>(ray: &Ray, hittable: H) -> Color {
    if let Some(rec) = hittable.hit(ray, 0., f32::INFINITY) {
        let normalized_color = 0.5f32
            * Vec3::new(
                rec.normal.i + 1f32,
                rec.normal.j + 1f32,
                rec.normal.k + 1f32,
            );

        return normalized_color
            .try_into()
            .expect("color is not normalized");
    }

    let unit_direction = ray.direction().unit();
    let a = 0.5f32 * (unit_direction.j + 1f32);

    let normalized_color =
        (1f32 - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.0);

    normalized_color
        .try_into()
        .expect("color is not normalized")
}

fn main() {
    // Logger
    let env = Env::default()
        .default_filter_or("trace")
        .default_write_style_or("always");
    env_logger::init_from_env(env);

    // Image
    let aspect_ratio = 16f32 / 9f32;
    let image_width = 400u32;
    let image_height = {
        let height = (image_width as f32 / aspect_ratio) as u32;

        if height < 1 {
            1
        } else {
            height
        }
    };

    // Camera
    let focal_length = 1f32;
    let viewport_height = 2.0f32;
    let viewport_width =
        viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical
    // viewport edges
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectros from pixel to pixel
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    let viewport_upper_left = camera_center
        - Vec3::new(0., 0., focal_length)
        - (viewport_u / 2)
        - (viewport_v / 2);
    let pixel00_loc =
        viewport_upper_left + 0.5f32 * (pixel_delta_u + pixel_delta_v);

    // World
    let mut world = World::new();
    world.add(Sphere::new((0., 0., -1.).into(), 0.5).unwrap());
    world.add(Sphere::new((0., -100.5, -1.).into(), 100.).unwrap());

    // Render
    let mut file = BufWriter::new(
        std::fs::File::options()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open("render.ppm")
            .expect("Could not create or open file `render.ppm`"),
    );

    write!(file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for j in 0..image_height {
        log::info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world);
            write_ppm_color(&mut file, color)
                .expect("could not write color to file");
        }
    }

    file.flush().expect("could not write buffer to file");
    log::info!("Done.");
}
