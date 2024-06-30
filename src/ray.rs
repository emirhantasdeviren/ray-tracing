use crate::interval::Interval;
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
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32, front_face: bool) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord>;
}

impl<T: Hittable> Hittable for &T {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        (*self).hit(ray, t_min, t_max)
    }
}

pub struct World(Vec<Box<dyn Hittable + 'static>>);

impl World {
    pub fn new() -> World {
        Self(vec![])
    }

    pub fn add<H>(&mut self, hittable: H)
    where
        H: Hittable + Clone + 'static,
    {
        self.0.push(Box::new(hittable.clone()))
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for hittable in self.0.iter() {
            if let Some(closest_rec) = hittable.hit(ray, t_min, closest_so_far)
            {
                closest_so_far = closest_rec.t;
                rec = Some(closest_rec);
            }
        }

        rec
    }
}
