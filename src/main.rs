mod movements;
mod shapes;
mod testing;

extern crate minifb;
extern crate rand;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_CIRCLES: usize = 10;
const LINE_THICKNESS: f32 = 2.0; // Thickness of the circle outline

fn main() {
    let mut window = Window::new(
        "Draw Empty Circles - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Randomly generate circles' parameters
    let mut rng = rand::thread_rng();
    let mut circles = Vec::new();
    for _ in 0..NUM_CIRCLES {
        let radius: f32 = rng.gen_range(20.0..100.0);
        let center_x: f32 = rng.gen_range(radius..(WIDTH as f32 - radius));
        let center_y: f32 = rng.gen_range(radius..(HEIGHT as f32 - radius));
        circles.push((center_x, center_y, radius));
    }

    // Draw the circle outlines
    for (center_x, center_y, radius) in &circles {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();

                // Check if the pixel is within the outline range
                if (distance >= *radius - LINE_THICKNESS) && (distance <= *radius + LINE_THICKNESS)
                {
                    buffer[y * WIDTH + x] = 0xFFFFFF; // White color for the circle outline
                }
            }
        }
    }

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
