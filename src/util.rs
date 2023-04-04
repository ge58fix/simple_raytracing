use nalgebra::Vector3;
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
