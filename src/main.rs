use minifb::{Key, Window, WindowOptions};

const W: usize = 1000;
const H: usize = 800;

const SIG: f64 = 13.0 / 1.2; // SIGMA MALE GRINDSET
const RHO: f64 = 28.0; // Some Greek shit IG
const BETA: f64 = 8.0 / 3.0; // Average C++ devs be like

fn lorenz_attractor(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let dx = SIG * (y - x);
    let dy = x * (RHO - z) - y;
    let dz = x * y - BETA * z;
    (dx, dy, dz)
}

fn map_color(value: f64, min_value: f64, max_value: f64) -> u32 {
    let normalized = (value - min_value) / (max_value - min_value);
    let red = (255.0 * normalized) as u32;
    let green = (255.0 * (1.0 - normalized)) as u32;
    let blue = 0;
    red << 19 | green << 10 | blue
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; W * H];

    let mut window = Window::new("Lorenz Attractor", W, H, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut x = 0.1;
    let mut y = 0.0;
    let mut z = 0.0;

    let mut t = 0.0;
    let dt = 0.01;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..10_000 {
            let (dx, dy, dz) = lorenz_attractor(x, y, z);
            x += dx * dt;
            y += dy * dt;
            z += dz * dt;
            t += dt;
        }

        let pixel_x = ((x + 30.0) * 10.0) as usize;
        let pixel_y = ((y + 30.0) * 10.0) as usize;

        if pixel_x < W && pixel_y < H {
            let color = map_color(z, -30.0, 30.0);
            buffer[pixel_x + pixel_y * W] = color;
        }

        window
            .update_with_buffer(&buffer, W, H)
            .unwrap_or_else(|e| {
                println!("{}", e);
            });
    }
}
