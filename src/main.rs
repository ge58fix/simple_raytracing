use crate::{
    camera::{create_camera, Camera},
    color::write_color,
    hittable::HitRecord,
    hittable_list::HittableList,
    sphere::Sphere,
};
use materials::{dielectric_scatter, lambertian_scatter, metal_scatter};
use nalgebra::Vector3;
use rand;
use ray::Ray;
use std::{collections::LinkedList, f32::INFINITY};
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod materials;
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
        material_num: 0,
        attenuation: Vector3::new(0., 0., 0.),
        mat_attribute: 1.,
    }; // placeholder

    if depth <= 0 {
        return Vector3::new(0., 0., 0.);
    }
    if world.clone().hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray {
            direction: Vector3::new(0., 0., 0.),
            origin: Vector3::new(0., 0., 0.),
        };
        let mut recc: HitRecord = rec.rec.clone();
        let indicator: bool;

        match rec.material_num {
            0 => indicator = lambertian_scatter(&mut recc, &mut scattered),
            1 => indicator = metal_scatter(r, &mut recc, &mut scattered, rec.mat_attribute),
            2 => indicator = dielectric_scatter(r, &mut recc, &mut scattered, rec.mat_attribute),
            _ => indicator = lambertian_scatter(&mut recc, &mut scattered),
        }
        if indicator {
            let vec: Vector3<f32> = ray_color(scattered, world, depth - 1);
            return Vector3::new(
                vec.x * rec.attenuation.x,
                vec.y * rec.attenuation.y,
                vec.z * rec.attenuation.z,
            );
        } else {
            return Vector3::new(0., 0., 0.);
        }
    }
    let unit_vec: Vector3<f32> = unit_vector(r.direction);
    let t = 0.5 * (unit_vec.y + 1.);
    return (1. - t) * Vector3::new(1., 1., 1.) + t * Vector3::new(0.5, 0.7, 1.);
}

fn main() {
    // Image

    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 50;

    // World

    let mut world: HittableList = HittableList {
        list: LinkedList::<Sphere>::new(),
    };
    let elt1 = Sphere {
        center: Vector3::new(0., -1000., 0.),
        radius: 1000.,
        rec: HitRecord::default(),
        material_num: 0,
        attenuation: Vector3::new(0.5, 0.5, 0.5),
        mat_attribute: 0.,
    };
    let elt2 = Sphere {
        center: Vector3::new(0., 1., 0.),
        radius: 1.0,
        rec: HitRecord::default(),
        material_num: 2,
        attenuation: Vector3::new(0.1, 0.2, 0.5),
        mat_attribute: 1.5,
    };

    let elt3 = Sphere {
        center: Vector3::new(-4., 1., 0.),
        radius: 1.,
        rec: HitRecord::default(),
        material_num: 0,
        attenuation: Vector3::new(0.9, 0.65, 0.1),
        mat_attribute: 1.,
    };
    let elt4 = Sphere {
        center: Vector3::new(4., 1., 0.),
        radius: 1.,
        rec: HitRecord::default(),
        material_num: 1,
        attenuation: Vector3::new(0.7, 0.6, 0.5),
        mat_attribute: 0.1,
    };

    let elt5 = Sphere {
        center: Vector3::new(4., 1., 2.),
        radius: 0.3,
        rec: HitRecord::default(),
        material_num: 1,
        attenuation: Vector3::new(0.7, 0.6, 0.5),
        mat_attribute: 1.5,
    };

    let elt6 = Sphere {
        center: Vector3::new(5., 1., 2.5),
        radius: 0.5,
        rec: HitRecord::default(),
        material_num: 0,
        attenuation: Vector3::new(0.5, 0., 0.3),
        mat_attribute: 1.,
    };

    world.list.push_back(elt1);
    world.list.push_back(elt2);
    world.list.push_back(elt3);
    world.list.push_back(elt4);
    world.list.push_back(elt5);
    world.list.push_back(elt6);

    // Camera

    let lookfrom = Vector3::new(13., 2., 3.);
    let lookat = Vector3::new(0., 0., 0.);
    let vup = Vector3::new(0., 1., 0.);
    //let focus_dist = (lookfrom - lookat).magnitude();
    let focus_dist = 10.0;
    let aperture = 0.1;

    let cam: Camera = create_camera(
        lookfrom,
        lookat,
        vup,
        ASPECT_RATIO,
        20.,
        focus_dist,
        aperture,
    );

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
