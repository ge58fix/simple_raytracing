
use std::collections::LinkedList;
use hittable::HitRecord;
use nalgebra::Vector3;
use crate::{hittable, ray::{Ray}, sphere};
use sphere::Sphere;

#[derive(Debug, Clone)]
pub struct HittableList {
    pub list : LinkedList<Sphere>,
}

impl HittableList {
    pub fn hit(self, r : Ray, t_min : f32, t_max : f32, rec : &mut Sphere) -> bool {
        let mut temp : Sphere = Sphere { center: Vector3::new(0.,0.,0.), radius: 0., rec: HitRecord::default()}; // placeholder
        let mut hit_indicator : bool = false;
        let mut current_closest : f32 = t_max;
        for sphere in self.list {
            let mut sphere_clone : Sphere = sphere.clone();
            if sphere_clone.hit(r, t_min, current_closest, &mut temp) {
                hit_indicator = true;
                current_closest = temp.rec.t;
                *rec = temp;
        }
    }
    return hit_indicator;
    }
}
