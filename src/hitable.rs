use std::fmt;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::{Material, HitRecord};

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere<M: Material + Clone> {
    center: Vec3,
    radius: f32,
    material: M,
}
impl<M: Material + Clone> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Sphere<M> {
        Sphere { center, radius, material }
    }
}
impl<M: Material + Clone> fmt::Display for Sphere<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sphere Center({}, {}, {}) Radius({})", 
            self.center.x, self.center.y, self.center.z,
            self.radius
        )
    }
}

impl<M: Material + Clone> Hitable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant <= 0.0 {
            return None;
        }
        let mut t = (-b - discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let vector_p = ray.point_at_t(t);
            return Some(HitRecord{
                t_factor: t,
                p_vect: vector_p,
                normal: (vector_p  - self.center) / self.radius,
                material: &self.material,
            })
        }
        t = (-b + discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let vector_p = ray.point_at_t(t);
            return Some(HitRecord{
                t_factor: t,
                p_vect: vector_p,
                normal: (vector_p  - self.center) / self.radius,
                material: &self.material,
            })
        }
        None
    }
}
pub struct HitList {
    pub hitlist: Vec<Box<dyn Hitable + Send + Sync + 'static>>,
}


impl<'a> Hitable for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in self.hitlist.iter() {
            if let Some(hit_temp) = hitable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_temp.t_factor;
                hit_anything = Some(hit_temp);
            }
        }
        hit_anything
    }
}
