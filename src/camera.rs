use std::io::{self, Write};

use crate::color::Color;
use crate::interval::Interval;
use crate::ray::{Hittable, Ray};
use crate::vec::Vec3;

pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1.0, 100)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;

        let center = (0., 0., 0.).into();

        let focal_length = 1.0f32;
        let viewport_height = 2.0f32;
        let viewport_width =
            viewport_height * (image_width as f32 / image_height as f32);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = center
            - Vec3::new(0., 0., focal_length)
            - viewport_u / 2
            - viewport_v / 2;

        let pixel00_loc =
            viewport_upper_left + 0.5f32 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render<H, W>(&self, hittable: H, mut write: W)
    where
        H: Hittable,
        W: Write,
    {
        write!(
            write,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .unwrap();

        for j in 0..self.image_height {
            log::info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i * self.pixel_delta_u)
                    + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = Self::ray_color(&ray, &hittable);
                Self::write_ppm_color(&mut write, color)
                    .expect("could not write color to file");
            }
        }

        write.flush().expect("could not write buffer to file");
        log::info!("Done.");
    }

    fn ray_color<H: Hittable>(ray: &Ray, hittable: H) -> Color {
        if let Some(rec) = hittable.hit(ray, Interval::new(0., f32::INFINITY)) {
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

    fn write_ppm_color<W: Write>(mut write: W, color: Color) -> io::Result<()> {
        writeln!(write, "{} {} {}", color.r, color.g, color.b)
    }
}
