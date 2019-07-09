use rand::prelude::*;
use std::f32::consts::PI;

use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Vec3, look_at: Vec3, view_up: Vec3, 
        vertical_fov: f32, aspect: f32, aperture: f32, focus_dist: f32
    ) -> Camera {
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let camera_direction = (look_from - look_at).make_unit_vector();
        let u = view_up.cross(camera_direction).make_unit_vector();
        let v = camera_direction.cross(u);
        Camera {
            origin : look_from,
            lower_left_corner: look_from - focus_dist * (half_width * u + half_height * v + camera_direction),
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height  * focus_dist * v,
            lens_radius: aperture / 2.0,
            u,
            v,
            w: camera_direction,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset
        )
    }
}