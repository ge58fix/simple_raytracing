use nalgebra::Vector3;

use crate::{sphere::Sphere, ray::Ray, util::{rand_unit_vector, near_zero, reflect}, hittable::HitRecord, unit_vector};

// I change the HitRecord only! Sphere needs to be updated with new value.
pub fn lambertian_scatter(r_in : Ray, rec : &mut HitRecord, scattered : &mut Ray) -> bool {
    let mut scatter_direction = rec.normal + rand_unit_vector();
    if near_zero(scatter_direction) {
        scatter_direction = rec.normal;
    }
    *scattered = Ray {origin: rec.p, direction: scatter_direction}; // unsure if this overwrites Ray correctly
    return true;
}

pub fn metal_scatter(r_in : Ray, rec : &mut HitRecord, scattered : &mut Ray) -> bool {
    let reflected : Vector3<f32> = reflect(unit_vector(r_in.direction), rec.normal);
    *scattered = Ray {origin : rec.p, direction: reflected};
    return scattered.direction.dot(&rec.normal) > 0.
}