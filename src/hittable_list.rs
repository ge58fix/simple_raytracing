use crate::{ray::Ray, sphere};
use sphere::Sphere;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct HittableList {
    pub list: LinkedList<Sphere>,
}

impl HittableList {
    pub fn hit(self, r: Ray, t_min: f32, t_max: f32, rec: &mut Option<Sphere>) -> bool {
        let mut hit_indicator: bool = false;
        let mut current_closest: f32 = t_max;
        let mut temp_opt: Option<Sphere> = None;
        for sphere in self.list {
            let mut sphere_clone: Sphere = sphere.clone();
            if sphere_clone.hit(r, t_min, current_closest, &mut temp_opt) {
                hit_indicator = true;
                current_closest = temp_opt.unwrap().rec.t;
                *rec = temp_opt;
            }
        }
        return hit_indicator;
    }
}
