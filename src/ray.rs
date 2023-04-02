use nalgebra::Vector3;

pub struct Ray {
    origin : Vector3<f32>,
    direction : Vector3<f32>,
}

pub fn create_ray(origin : Vector3<f32>, direction : Vector3<f32>) -> Ray {
    Ray {
        origin,
        direction,
    }
}
impl Ray {
    pub fn at(&mut self, t : f32) -> Vector3<f32> {
        return self.origin + t * self.direction;
    }
}