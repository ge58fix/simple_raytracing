use nalgebra::Vector3;

fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        return min;
    } else if v > max {
        return max;
    } else {
        return v;
    }
}

pub fn write_color(pixel_color: Vector3<f32>, samples_per_pixel: u32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
    let scale = 1. / samples_per_pixel as f32;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    println!(
        "{} {} {}",
        (256. * clamp(r, 0., 0.999)) as u32,
        (256. * clamp(g, 0., 0.999)) as u32,
        (256. * clamp(b, 0., 0.999)) as u32
    );
}
