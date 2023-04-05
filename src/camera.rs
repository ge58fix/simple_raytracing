use nalgebra::Vector3;

use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

pub fn create_camera() -> Camera {
    let origin: Vector3<f32> = Vector3::new(0., 0., 0.);
    let aspect_ratio: f32 = 16. / 9.;
    let viewport_height: f32 = 2.;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.;
    let horizontal: Vector3<f32> = Vector3::new(viewport_width, 0., 0.);
    let vertical: Vector3<f32> = Vector3::new(0., viewport_height, 0.);
    let lower_left_corner: Vector3<f32> =
        origin - horizontal / 2. - vertical / 2. - Vector3::new(0., 0., focal_length);

    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
    }
}

impl Camera {
    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
