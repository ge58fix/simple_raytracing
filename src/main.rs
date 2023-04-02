use nalgebra::Vector3;
mod ray;

fn main() {
    // Image

    const WIDTH : u32 = 256;
    const HEIGHT : u32 = 256;

    // Render

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..(HEIGHT)).rev() {
        eprintln!("\rRows remaining: {}", (j + 1));
        for i in 0..(WIDTH) {
            let color : Vector3<f32> = Vector3::new(i as f32 / (WIDTH - 1) as f32, j as f32 / (HEIGHT - 1) as f32, 0.25);
            write_color(color);
        }
    }
    println!("\nComplete.\n");
}

fn write_color(color : Vector3<f32>) {
    println!("{} {} {}\n", (255.999 * color[0]) as u32, (255.999 * color[1]) as u32, (255.999 * color[2]) as u32)
}
