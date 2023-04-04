use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn at(self, t: f32) -> Vector3<f32> {
        return self.origin + t * self.direction;
    }
}
