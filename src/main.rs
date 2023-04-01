fn main() {
    // Image

    const WIDTH : u32 = 256;
    const HEIGHT : u32 = 256;

    // Render

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..(HEIGHT)).rev() {
        for i in 0..(WIDTH) {
            let r : f64 = i as f64 / (WIDTH - 1) as f64;
            let g : f64 = j as f64 / (HEIGHT - 1) as f64;
            let b : f64 = 0.25;

            let ir : i32 = (255.999 * r) as i32;
            let ig : i32 = (255.999 * g) as i32;
            let ib : i32 = (255.999 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    println!("\nComplete.\n");
}
