use rand::prelude::*;

use crate::vec3::Vec3;
use crate::color::Color;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t_factor: f32,
    pub p_vect: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub struct Scatter {
    pub color: Color,
    pub ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = (2.0 * 
            Vec3::new(
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0)
            )) -  Vec3::new_unit();
        if p.square_length() < 1.0 {
            return p;
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let target = hit_record.p_vect + hit_record.normal + random_in_unit_sphere();
        Some(Scatter{
            color: self.albedo,
            ray: Some(Ray::new(hit_record.p_vect, target - hit_record.p_vect)),
        })
    }
}

fn reflect(a: &Vec3, b: &Vec3) -> Vec3 {
    *a - 2.0 * a.dot(*b) * *b
}


#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(a: Color, f: f32) -> Metal {
        Metal { albedo: a, fuzz: if f > 1.0 { 1.0 } else {f} }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(hit_record.p_vect, reflected + self.fuzz * random_in_unit_sphere());
        Some(Scatter{
            color: self.albedo,
            ray: if scattered.direction.dot(hit_record.normal) > 0.0 { Some(scattered) } else { None },
        })
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_t: f32) -> Option<Vec3> {
    let unit_vector = (*v).make_unit_vector();
    let dt = unit_vector.dot(*n);
    let discriminant: f32 = 1.0 - ni_over_t * ni_over_t * (1.0 - dt*dt);
    if discriminant > 0.0 {
        Some(ni_over_t * (unit_vector - (*n) * dt) - (*n) * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32{
    let mut r0: f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f32
}
impl Dielectric {
    pub fn new(index: f32) -> Dielectric {
        Dielectric { ref_idx: index }
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let outward_normal: Vec3;
        let ni_over_t: f32;
        let cosine: f32;
        
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let attenuation = Color::white();
        let mut rng = thread_rng();

        if ray.direction.dot(hit_record.normal) > 0.0 {
            outward_normal = - hit_record.normal;
            ni_over_t = self.ref_idx;
            cosine = self.ref_idx * ray.direction.dot(hit_record.normal) / ray.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_t = 1.0 / self.ref_idx;
            cosine = - ray.direction.dot(hit_record.normal) / ray.direction.length();
        }
        if let Some(refracted) = refract(&ray.direction, &outward_normal, ni_over_t) {
            if rng.gen_range(0.0, 1.0) > schlick(cosine, self.ref_idx){
                return Some(Scatter { 
                    color: attenuation,
                    ray: Some(Ray::new(hit_record.p_vect, refracted))
                });
            }
        }
        Some(Scatter {
                color: attenuation,
                ray: Some(Ray::new(hit_record.p_vect, reflected))
            })
    }
}
