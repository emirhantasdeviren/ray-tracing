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
}
