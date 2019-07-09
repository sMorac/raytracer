use rand::prelude::*;

mod vec3;
mod color;
mod hitable;
mod ray;
mod camera;
mod material;

use camera::Camera;
use vec3::Vec3;
use color::Color;
use color::ColorU8;
use ray::Ray;
use hitable::Hitable;
use hitable::Sphere;
use hitable::HitList;
use material::Metal;
use material::Lambertian;
use material::Dielectric;

use std::thread;
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;

fn render_color(
    resolution: u16,
    i: f32,
    j: f32,
    x_size: f32,
    y_size: f32,
    camera: &Camera,
    random_scene: &HitList
) -> ColorU8 {
    let mut rng = thread_rng();
    let mut rendered_color = Color::black();
    for _res in 0..resolution {
        let u = (i + rng.gen_range(0.0, 1.0)) / x_size;
        let v = (j + rng.gen_range(0.0, 1.0)) / y_size;
        let r = camera.get_ray(u, v);
        rendered_color += color(&r, random_scene, 0);
    }
    rendered_color /= resolution as f32;
    rendered_color = rendered_color.sqrt();
    rendered_color *= f32::from(std::u8::MAX);
    ColorU8::make_from_color(rendered_color)
}

fn color<T: Hitable>(ray: &Ray, world: &T, depth: u8) -> Color {
    if let Some(hit_record) = world.hit(ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) { 
                if let Some(scattered_ray) = scatter.ray {
                    return scatter.color * color(&scattered_ray, world, depth + 1);
                }
            } else {
                return Color::black();
            }
        } else {
            return Color::black();
        }
    }
    let unit_direction = ray.direction.make_unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::white() + t * Color{ red: 0.5, green: 0.7, blue: 1.0 }
}

pub fn random_scene() -> HitList {
    let mut rng = thread_rng();
    let mut world = HitList{ hitlist: Vec::new() };
    let obj0 = Sphere::new(
        Vec3::new(0.0, 0.0, -1000.0),
        1000.0,
        Lambertian::new(Color{ red: 1.0, green: 0.6, blue: 0.5 })
    );
    world.hitlist.push(Box::new(obj0));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen_range(0.0, 1.0);
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0, 1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.hitlist.push(
                        Box::new(
                            Sphere::new(center, 0.2, Lambertian::new(
                                Color {
                                    red: rng.gen_range(0.0, 1.0)*rng.gen_range(0.0, 1.0),
                                    green: rng.gen_range(0.0, 1.0)*rng.gen_range(0.0, 1.0),
                                    blue: rng.gen_range(0.0, 1.0)*rng.gen_range(0.0, 1.0)
                                })
                            )
                        )
                    );
                } else if choose_mat < 0.95 {
                    world.hitlist.push(
                        Box::new(
                            Sphere::new(
                                center,
                                0.2,
                                Metal::new(
                                    Color {
                                        red: 0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                                        green: 0.5 * (1.0 + rng.gen_range(0.0, 1.0)),
                                        blue: 0.5 * (1.0 + rng.gen_range(0.0, 1.0))
                                    },
                                    0.5 * rng.gen_range(0.0, 1.0) 
                                ),
                            )
                        )
                    );
                } else {
                   world.hitlist.push(
                        Box::new(
                            Sphere::new(
                                center,
                                0.2,
                                Dielectric::new(1.5),
                            )
                        )
                    );
                }
            }
        }
    }
    world.hitlist.push(Box::new(
                            Sphere::new(
                                Vec3::new(0.0, 0.0, 2.0),
                                2.0,
                                Dielectric::new(1.5))
                            )
                       );
    world.hitlist.push(Box::new(
                            Sphere::new(
                                Vec3::new(-4.0, 0.0, 2.0),
                                2.0,
                                Lambertian::new(Color { red: 0.6, green: 0.2, blue: 0.2 })
                            )
                        ));
    world.hitlist.push(Box::new(
                        Sphere::new(
                            Vec3::new(4.0, 0.0, 2.0),
                            2.0,
                            Metal::new(
                                Color {
                                    red: 0.85,
                                    green: 0.9,
                                    blue: 0.7
                                },
                                0.0
                            )
                        )
                    ));

    world
}


pub fn print_image() {
    
    let x_size = 500;
    let x_size_f: f32 = 500.0;
    let y_size = 400;
    let y_size_f: f32 = 400.0;
    let s_size =  100; 
    println!("P3");
    println!("{} {}", x_size, y_size);
    println!("255");
    let sphere1 = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(
            Color { red: 0.1, green: 0.2, blue: 0.5 }
        )
    );
    let sphere2 = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(
            Color { red: 0.8, green: 0.8, blue: 0.0 }
        )
    );
    let sphere3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::new(Color{ red: 0.8, green: 0.6, blue: 0.2 }, 0.3));
    let sphere4 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5));
    let sphere5 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Dielectric::new(1.5));
    let mut world = HitList{ hitlist: Vec::new() };
    world.hitlist.push(Box::new(sphere1));
    world.hitlist.push(Box::new(sphere2));
    world.hitlist.push(Box::new(sphere3));
    world.hitlist.push(Box::new(sphere4));
    world.hitlist.push(Box::new(sphere5));
    let random_scene = Arc::new(random_scene());

    let look_from = Vec3::new(20.0 * 0.47f32.cos(), 20.0 * 0.47f32.sin(), 3.0);
    let look_at = Vec3::new(0.0, 0.0, 1.0);
    let dist_to_focus = (look_from - look_at).length();
    let view_up = Vec3::new(0.0, 0.0, 1.0);
    let aperture: f32 = 0.3;
    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        x_size_f/y_size_f,
        aperture,
        dist_to_focus
    ));

    let mut coordinates = Vec::new();
    for i in 0..x_size {
        for j in 0..y_size {
            coordinates.push((i, j));
        }
    }
    let size_coordinates = coordinates.len();
    let size_slice = size_coordinates / 8;
    let rest = size_coordinates % 8;
    let mut comp = Vec::new() ;
    let mut processing = Vec::new() ;
    for _num_thread in 0..8 {
        comp.push(Mutex::new(HashMap::new()));
        processing.push(Mutex::new(false));
    }
    let computation = Arc::new(comp);
    let processing_arc = Arc::new(processing);
    let share_coordinates = Arc::new(coordinates);
    let mut handles = vec![];
    for num_thread in 0..8 {
        let computation_local = Arc::clone(&computation);
        let local_coordinates = Arc::clone(&share_coordinates);
        let local_camera = Arc::clone(&camera);
        let local_rs = Arc::clone(&random_scene);
        let local_processing = Arc::clone(&processing_arc);
        let handle = thread::spawn(move || {
            let mut map = computation_local[num_thread].lock().unwrap();
            let start = num_thread * size_slice;
            let mut end = (num_thread + 1) * size_slice - 1;
            if num_thread == 7 {
                end += rest;
            }
            // println!("n{} s{} e{} l{}", num_thread, start, end, size_coordinates);
            for (i, j) in &local_coordinates[start..end+1] {
                map.insert((*i, *j), 
                    render_color(
                        s_size, 
                        *i as f32,
                        *j as f32,
                        x_size as f32,
                        y_size as f32,
                        &local_camera,
                        &local_rs,
                    )
                );
            }
            let mut set_done = local_processing[num_thread].lock().unwrap();
            *set_done = true;
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut map = HashMap::new();
    for num_thread in 0..8 {
        let current_map = computation[num_thread].lock().unwrap().clone().into_iter();
        map.extend(
            current_map.map(|(k, v)| (k.clone(), v.clone()))
        );
    }
    // println!("{}", map.len());
    for j in (0..y_size).rev() {
        for i in 0..x_size {
            match map.get(&(i,j)) {
                Some(color) => println!("{} {} {}", color.red, color.blue, color.green),
                None => println!("{} {} not found.", i, j)
            }
        }
    }
}
