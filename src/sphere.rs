use crate::ray::{HitRecord, Hittable, Ray};
use crate::vec::Vec3;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Result<Self, &'static str> {
        if radius < 0f32 {
            Err("radius can not be negative")
        } else {
            Ok(Self { center, radius })
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h * h - a * c;

        if discriminant < 0f32 {
            return None;
        }

        let d_sqrt = discriminant.sqrt();

        let root = {
            let mut root = (h - d_sqrt) / a;
            if root <= t_min || root >= t_max {
                root = (h + d_sqrt) / a;
                if root <= t_min || root >= t_max {
                    None
                } else {
                    Some(root)
                }
            } else {
                Some(root)
            }
        };

        root.map(|root| {
            let t = root;
            let point = ray.at(t);
            let out_normal = (point - self.center) / self.radius;
            let front_face = ray.direction().dot(&out_normal).is_sign_negative();
            let normal = if front_face {
                out_normal
            } else {
                -out_normal
            };

            HitRecord::new(point, normal, t, front_face)
        })
    }
}
