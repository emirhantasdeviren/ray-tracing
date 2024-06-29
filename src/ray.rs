use crate::vec::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn hits_sphere(&self, center: &Vec3, radius: f32) -> Option<f32> {
        let oc = center - self.origin();
        let a = self.direction().dot(self.direction());
        let b = -2f32 * self.direction().dot(&oc);
        let c = oc.dot(&oc) - radius * radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2f32 * a))
        }
    }
}

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f32,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32) -> Self {
        Self { point, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
