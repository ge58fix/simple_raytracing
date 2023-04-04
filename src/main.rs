use crate::{
    camera::{create_camera, Camera},
    color::write_color,
    hittable::HitRecord,
    hittable_list::HittableList,
    sphere::Sphere,
};
use nalgebra::Vector3;
use rand;
use ray::Ray;
use std::{collections::LinkedList, f32::INFINITY};
use util::{rand_in_unit_sphere, rand_unit_vector, rand_in_hemisphere};
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;

fn unit_vector(v: Vector3<f32>) -> Vector3<f32> {
    return v / v.magnitude();
}

fn ray_color(r: Ray, world: HittableList, depth: u32) -> Vector3<f32> {
    let mut rec: Sphere = Sphere {
        center: Vector3::new(0., 0., 0.),
        radius: 0.,
        rec: HitRecord::default(),
    }; // placeholder

    if depth <= 0 {
        return Vector3::new(0.001, 0., 0.);
    }
    if world.clone().hit(r, 0., INFINITY, &mut rec) {
        let target = rec.rec.p + rand_in_hemisphere(rec.rec.normal);
        return 0.5
            * ray_color(
                Ray {
                    origin: rec.rec.p,
                    direction: target - rec.rec.p,
                },
                world,
                depth - 1,
            );
    }
    let unit_vec: Vector3<f32> = unit_vector(r.direction);
    let t = 0.5 * (unit_vec.y + 1.);
    return (1. - t) * Vector3::new(1., 1., 1.) + t * Vector3::new(0.5, 0.7, 1.);
}

fn main() {
    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World

    let mut world: HittableList = HittableList {
        list: LinkedList::<Sphere>::new(),
    };
    let elt1 = Sphere {
        center: Vector3::new(0., 0.0, -1.),
        radius: 0.5,
        rec: HitRecord::default(),
    };
    let elt2 = Sphere {
        center: Vector3::new(0., -100.5, -1.),
        radius: 100.,
        rec: HitRecord::default(),
    };
    world.list.push_back(elt1);
    world.list.push_back(elt2);

    // Camera

    let cam: Camera = create_camera();

    // Render

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..(HEIGHT)).rev() {
        eprintln!("\rRows remaining: {}", (j + 1));
        for i in 0..WIDTH {
            let mut pixel_color: Vector3<f32> = Vector3::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (i as f32 + rand::random::<f32>()) / (WIDTH - 1) as f32;
                let v: f32 = (j as f32 + rand::random::<f32>()) / (HEIGHT - 1) as f32;
                let r: Ray = cam.clone().get_ray(u, v);
                pixel_color += ray_color(r, world.clone(), MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nComplete.\n");
}
