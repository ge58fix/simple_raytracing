use crate::ray;
use nalgebra::Vector3;
use ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    //pub mat : u8,
}

impl HitRecord {
    pub fn set_face_normal(mut self, r: Ray, outward_normal: Vector3<f32>) -> Vector3<f32> {
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        if self.front_face {
            return outward_normal;
        } else {
            return -outward_normal;
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        }
    }
}
