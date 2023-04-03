
use nalgebra::Vector3;
use ray::Ray;
use hittable::HitRecord;
use crate::{hittable, ray};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center : Vector3<f32>,
    pub radius : f32,
    pub rec : HitRecord,
}

impl Sphere {
    pub fn hit(&mut self, r : Ray, t_min : f32, t_max : f32, rec : &mut Sphere) -> bool {
        let difference : Vector3<f32> = r.origin - self.center;
        let a : f32 = r.direction.magnitude_squared();
        let h : f32 = difference.dot(&r.direction);
        let c : f32 = difference.magnitude_squared() - self.radius * self.radius;
        let discriminant : f32 = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd : f32 = discriminant.sqrt();

        let mut val : f32 = (-h - sqrtd) / a;
        if val < t_min || val > t_max {
            val = (-h + sqrtd) / a;
            if val < t_min || val > t_max {
                return false;
            }
        }
        rec.rec.t = val;
        rec.rec.p = r.at(rec.rec.t);
        let outward_normal : Vector3<f32> = (rec.rec.p - self.center) / self.radius;
        rec.rec.normal = rec.rec.set_face_normal(r, outward_normal);

        return true;
    }
}