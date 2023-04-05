use nalgebra::Vector3;
use rand::random;

use crate::{ray::Ray, util::{rand_unit_vector, near_zero, reflect, rand_in_unit_sphere, refract}, hittable::HitRecord, unit_vector};

pub fn lambertian_scatter(rec : &mut HitRecord, scattered : &mut Ray) -> bool {
    let mut scatter_direction = rec.normal + rand_unit_vector();
    if near_zero(scatter_direction) {
        scatter_direction = rec.normal;
    }
    *scattered = Ray {origin: rec.p, direction: scatter_direction};
    return true;
}

pub fn metal_scatter(r_in: Ray, rec : &mut HitRecord, scattered : &mut Ray, fuzz: f32) -> bool {
    let reflected : Vector3<f32> = reflect(unit_vector(r_in.direction), rec.normal);
    *scattered = Ray {origin : rec.p, direction: reflected + fuzz * rand_in_unit_sphere()};
    return scattered.direction.dot(&rec.normal) > 0.
}

pub fn dielectric_scatter(r_in : Ray, rec : &mut HitRecord, scattered : &mut Ray, refraction_index: f32) -> bool {
    let mut refraction_ratio: f32 = 0.;
    if rec.front_face {
        refraction_ratio = refraction_index / 1.;
    }
    else {
        refraction_ratio = refraction_index;
    }

    let unit_direction = unit_vector(r_in.direction);
    let cos_theta = f32::min((- unit_direction).dot(&rec.normal), 1.);
    let sine_theta = (1. - cos_theta * cos_theta).sqrt();
    let cannot_refract = refraction_ratio * sine_theta > 1.;
    let mut direction = Vector3::new(0., 0., 0.);
    if cannot_refract || reflectance(cos_theta, refraction_index) > random() {
        direction = reflect(unit_direction, rec.normal);
    }
    else {
        direction = refract(unit_direction, rec.normal, refraction_ratio);
    }
    *scattered = Ray {origin: rec.p, direction: direction};
    return true;
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 *= r0;
    return r0 + (1. - r0) * (1. - cosine).powi(5);
}