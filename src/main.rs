mod movements;
mod shapes;
mod testing;

extern crate minifb;
extern crate rand;

use minifb::{Key, Window, WindowOptions};
use shapes::circle::CircleChain;
use shapes::combi_shapes::seven_chain;
use testing::test_circle::{test_circle, test_circle_chain};
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const LINE_THICKNESS: f32 = 1.0; // Thickness of the circle outline

fn main() {
    test_circle();
    test_circle_chain();
    let mut circle_chain = seven_chain();

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

    let mut circles = Vec::new();
    circle_chain.move_head(-150.0, -20.0);
    circle_chain.allign_nodes();
    append_new_circles(&mut circles, &circle_chain);

    circle_chain.move_head(-250.0, -80.0);
    circle_chain.allign_nodes();
    append_new_circles(&mut circles, &circle_chain);

    circle_chain.move_head(40.0, -161.0);
    circle_chain.allign_nodes();
    append_new_circles(&mut circles, &circle_chain);

    circle_chain.move_head(200.0, -181.0);
    circle_chain.allign_nodes();
    append_new_circles(&mut circles, &circle_chain);

    for (center_x, center_y, radius) in &circles {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();

                // Check if the pixel is within the outline range
                if (distance >= *radius - LINE_THICKNESS) && (distance <= *radius + LINE_THICKNESS)
                {
                    buffer[y * WIDTH + x] = 0xAAAAAA; // White color for the circle outline
                }
            }
        }
    }

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn append_new_circles(circles: &mut Vec<(f32, f32, f32)>, circle_chain: &CircleChain) {
    for ch in &circle_chain.circles {
        let radius: f32 = ch.radius as f32;
        let center_x: f32 = ch.x as f32;
        let center_y: f32 = ch.y as f32;
        circles.push((center_x, center_y, radius));
    }
}
