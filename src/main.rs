use nalgebra::Vector3;
use ray::Ray;
use crate::ray::create_ray;
mod ray;

fn unit_vector(v : Vector3<f32>) -> Vector3<f32> {
    return v / v.magnitude();
}

fn sphere_hit(origin : Vector3<f32>, radius : f32, r : &Ray) -> f32 {
    let difference : Vector3<f32> = r.origin - origin;
    let a : f32 = r.direction.dot(&r.direction);
    let b : f32 = 2.0 * difference.dot(&r.direction);
    let c : f32 = difference.dot(&(difference)) - radius * radius;
    let discriminant : f32 = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    else {
        return (-b - discriminant.sqrt()) / (2.0 * a)
    }
}


fn ray_color(r : Ray) -> Vector3<f32> {
    let origin : Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    let mut t : f32 = sphere_hit(origin, 0.5, &r);
    if t > 0.0 {
        let n : Vector3<f32> = unit_vector(r.at(t) - origin);
        return 0.5 * Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_vec : Vector3<f32> = unit_vector(r.direction);
    t = 0.5 * (unit_vec.y + 1.0);
    return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const WIDTH : u32 = 400;
    const HEIGHT : u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera

    let viewport_height : f32 = 2.0;
    let viewport_width : f32 = ASPECT_RATIO * viewport_height;
    let focal_length : f32 = 1.0;
    let origin : Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    let horizontal : Vector3<f32> = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical : Vector3<f32> = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner : Vector3<f32> = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..(HEIGHT)).rev() {
        eprintln!("\rRows remaining: {}", (j + 1));
        for i in 0..(WIDTH) {
            let u : f32 = i as f32 / (WIDTH - 1) as f32;
            let v : f32 = j as f32 / (HEIGHT - 1) as f32;
            let r : Ray = create_ray(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let color : Vector3<f32> = ray_color(r);
            write_color(color);
        }
    }
    println!("\nComplete.\n");
}

fn write_color(color : Vector3<f32>) {
    println!("{} {} {}\n", (255.999 * color[0]) as u32, (255.999 * color[1]) as u32, (255.999 * color[2]) as u32)
}
