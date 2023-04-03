
use ray::Ray;
use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p : Vector3<f32>,
    pub normal : Vector3<f32>,
    pub t : f32,
    pub front_face : bool,
}

impl HitRecord {
    pub fn set_face_normal(mut self, r : Ray, outward_normal : Vector3<f32>) {
        front_face = r.direction.dot(outward_normal) < 0;
        if (front_face) {
            normal = outward_normal;
        }
        else {
            normal = - outward_normal;
        }
    }

}