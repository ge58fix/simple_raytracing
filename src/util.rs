
use std::{cmp::min, f32::consts::PI};

use nalgebra::{Vector3, ComplexField};
use rand::{self, Rng};

use crate::unit_vector;

fn rand_vec3() -> Vector3<f32> {
    Vector3::new(
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    )
}

fn rand_vec3_in_range(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        rand::thread_rng().gen_range(min..max),
        rand::thread_rng().gen_range(min..max),
        rand::thread_rng().gen_range(min..max),
    )
}

fn rand_float_in_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn rand_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = rand_vec3_in_range(-1.0, 1.0);
        if p.magnitude_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn rand_unit_vector() -> Vector3<f32> {
    return unit_vector(rand_in_unit_sphere());
}

pub fn rand_in_hemisphere(normal : Vector3<f32>) -> Vector3<f32> {
    let in_sphere = rand_in_unit_sphere();
    if (in_sphere.dot(&normal)) > 0.0 {
        return in_sphere;
    }
    else {
        return - in_sphere;
    }
}

pub fn near_zero(v : Vector3<f32>) -> bool {
    let e : f32 = 1e-8;
    return v.x.abs() < e && v.y.abs() < e && v.z.abs() < e
}

pub fn reflect(v : Vector3<f32>, n : Vector3<f32>) -> Vector3<f32> {
    return v - 2. * v.dot(&n) * n;
}

pub fn refract(uv: Vector3<f32>, n: Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = f32::min((-uv).dot(&n), 1.);
    let r_out_perpendicular = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = - (1. - r_out_perpendicular.magnitude_squared()).abs().sqrt() * n;
    return r_out_perpendicular + r_out_parallel;
}

pub fn degrees_to_radians(x: f32) -> f32 {
    return x * PI / 180.
}

pub fn rand_in_unit_disk() -> Vector3<f32> {
    loop {
        let mut p = rand_vec3_in_range(-1., 1.);
        p.z = 0.;
        if p.magnitude_squared() >= 1. {
            continue;
        }
        return p;
    }
}
