
use nalgebra::Vector3;
use ray::Ray;
use hittable::HitRecord;

pub struct Sphere {
    pub center : Vector3<f32>,
    pub radius : f32,
}

impl Sphere {
    pub fn hit(r : Ray, t_min : f32, t_max : f32, mut rec : HitRecord) -> bool {
        let difference : Vector3<f32> = r.origin - origin;
        let a : f32 = r.direction.dot(&r.direction);
        let h : f32 = difference.dot(&r.direction);
        let c : f32 = difference.dot(&(difference)) - radius * radius;
        let discriminant : f32 = h * h - a * c;
        if (discriminant < 0) {
            return false;
        }
        let sqrtd : f32 = discriminant.sqrt();
        let mut val : f32 = (-h - sqrtd) / a;
        if (val < t_min || val > t_max) {
            val = (-h + sqrtd) / a;
            if (val < t_min || val > t_max) {
                return false;
            }
            rec.t = val;
            rec.p = r.at(rec.t);
            let outward_normal : Vector3<f32> = (rec.p - center) / radius;
            rec.set_face_normal(r, outward_normal);
            
            return true;
        }
    }
}