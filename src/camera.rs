use nalgebra::Vector3;

use crate::{ray::Ray, util::{degrees_to_radians, rand_in_unit_disk}, unit_vector};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    lens_radius: f32,
    u: Vector3<f32>,
    v: Vector3<f32>
}

pub fn create_camera(lookfrom: Vector3<f32>, lookat: Vector3<f32>, vup: Vector3<f32>, aspect_ratio: f32, vfov: f32, focus_dist: f32, aperture: f32) -> Camera {
    let theta = degrees_to_radians(vfov);
    let h = (theta / 2.).tan();
    let viewport_height: f32 = 2. * h;
    let viewport_width: f32 = aspect_ratio * viewport_height;

    let w = unit_vector(lookfrom - lookat);
    let u = unit_vector(vup.cross(&w));
    let v = w.cross(&u);

    let origin: Vector3<f32> = lookfrom;
    let horizontal: Vector3<f32> = focus_dist * viewport_width * u;
    let vertical: Vector3<f32> = focus_dist * viewport_height * v;
    let lower_left_corner: Vector3<f32> =
        origin - horizontal / 2. - vertical / 2. - focus_dist * w;
    let lens_radius = aperture / 2.;

    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
        lens_radius,
        u, v,
    }
}

impl Camera {
    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * rand_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin - offset,
        }
    }
}
