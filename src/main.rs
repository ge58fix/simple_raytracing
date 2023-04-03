use std::{collections::LinkedList, f32::INFINITY};
use nalgebra::Vector3;
use ray::Ray;
use crate::{ray::create_ray, hittable_list::HittableList, sphere::Sphere, hittable::HitRecord};
mod ray;
mod hittable_list;
mod hittable;
mod sphere;

fn unit_vector(v : Vector3<f32>) -> Vector3<f32> {
    return v / v.magnitude();
}

fn ray_color(r : Ray, world : HittableList) -> Vector3<f32> {
    let mut rec : Sphere = Sphere { center: Vector3::new(0.,0.,0.), radius: 0., rec: HitRecord::default()}; //placeholder

    if world.hit(r, 0., INFINITY, &mut rec) {
        return 0.5 * (rec.rec.normal + Vector3::new(1., 1., 1.));
    }
    let unit_vec : Vector3<f32> = unit_vector(r.direction);
    let t = 0.5 * (unit_vec.y + 1.);
    return (1. - t) * Vector3::new(1., 1., 1.) + t * Vector3::new(0.5, 0.7, 1.)
}

fn main() {
    // Image

    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const WIDTH : u32 = 400;
    const HEIGHT : u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    // World

    let mut world : HittableList = HittableList {list : LinkedList::<Sphere>::new()};
    let elt1 = Sphere {center : Vector3::new(0., 0.0, -1.), radius : 0.5, rec : HitRecord::default()};
    let elt2 = Sphere {center : Vector3::new(0., -100.5, -1.), radius : 100., rec : HitRecord::default()};
    world.list.push_back(elt1);
    world.list.push_back(elt2);

    // Camera

    let viewport_height : f32 = 2.0;
    let viewport_width : f32 = ASPECT_RATIO * viewport_height;
    let focal_length : f32 = 1.0;
    let origin : Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    let horizontal : Vector3<f32> = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical : Vector3<f32> = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner : Vector3<f32> = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..(HEIGHT)).rev() {
        eprintln!("\rRows remaining: {}", (j + 1));
        for i in 0..(WIDTH) {
            let u : f32 = i as f32 / (WIDTH - 1) as f32;
            let v : f32 = j as f32 / (HEIGHT - 1) as f32;
            let r : Ray = create_ray(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let color : Vector3<f32> = ray_color(r, world.clone());
            write_color(color);
        }
    }
    println!("\nComplete.\n");
}

fn write_color(color : Vector3<f32>) {
    println!("{} {} {}\n", (255.999 * color[0]) as u32, (255.999 * color[1]) as u32, (255.999 * color[2]) as u32)
}
