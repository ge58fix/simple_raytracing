use crate::{hittable, ray};
use hittable::HitRecord;
use nalgebra::Vector3;
use ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub rec: HitRecord,
    pub material_num: u8,
    pub attenuation: Vector3<f32>,
    pub mat_attribute: f32,
}

impl Sphere {
    pub fn hit(&mut self, r: Ray, t_min: f32, t_max: f32, sphere_opt: &mut Option<Sphere>) -> bool {
        let difference: Vector3<f32> = r.origin - self.center;
        let a: f32 = r.direction.magnitude_squared();
        let h: f32 = difference.dot(&r.direction);
        let c: f32 = difference.magnitude_squared() - self.radius * self.radius;
        let discriminant: f32 = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f32 = discriminant.sqrt();

        let mut t: f32 = (-h - sqrtd) / a;
        if t < t_min || t > t_max {
            t = (-h + sqrtd) / a;
            if t < t_min || t > t_max {
                return false;
            }
        }

        let mut sphere = Sphere {
            center: self.center,
            radius: self.radius,
            rec: HitRecord::default(),
            material_num: self.material_num,
            attenuation: self.attenuation,
            mat_attribute: self.mat_attribute};
        sphere.rec.t = t;
        sphere.rec.p = r.at(t);
        let outward_normal: Vector3<f32> = (sphere.rec.p - self.center) / self.radius;
        sphere.rec.normal = sphere.rec.set_face_normal(r, outward_normal);
        *sphere_opt = Some(sphere);
        return true;
    }
}
