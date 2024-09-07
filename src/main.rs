mod movements;
mod shapes;
mod testing;

extern crate minifb;
extern crate rand;

use minifb::{Key, Window, WindowOptions};
use shapes::circle::{CircleChain, CircularHelper};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const LINE_THICKNESS: f32 = 1.0; // Thickness of the circle outline

fn main() {
    let head = CircularHelper::new(400.0, 300.0, 10.0, 0.0);
    let mut circle_chain = CircleChain::new(&head, 40.0);
    let circle1 = CircularHelper::new(460.0, 360.0, 15.0, 0.0);
    let circle2 = CircularHelper::new(660.0, 300.0, 20.0, 0.0);
    let circle3 = CircularHelper::new(660.0, 500.0, 10.0, 0.0);
    let circle4 = CircularHelper::new(660.0, 200.0, 10.0, 0.0);
    let circle5 = CircularHelper::new(660.0, 400.0, 5.0, 0.0);
    let circle6 = CircularHelper::new(660.0, 600.0, 5.0, 0.0);
    circle_chain.add_circle(&circle1);
    circle_chain.add_circle(&circle2);
    circle_chain.add_circle(&circle3);
    circle_chain.add_circle(&circle4);
    circle_chain.add_circle(&circle5);
    circle_chain.add_circle(&circle6);
    circle_chain.allign_nodes();

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
    let mut circles = Vec::new();
    circle_chain.move_head(-80.0, -61.0);
    circle_chain.allign_nodes();

    circles.push((
        circle_chain.head.x as f32,
        circle_chain.head.y as f32,
        circle_chain.head.radius as f32,
    ));
    for ch in &circle_chain.circles {
        let radius: f32 = ch.radius as f32;
        let center_x: f32 = ch.x as f32;
        let center_y: f32 = ch.y as f32;
        circles.push((center_x, center_y, radius));
    }

    circle_chain.move_head(0.0, -181.0);
    circle_chain.allign_nodes();
    circles.push((
        circle_chain.head.x as f32,
        circle_chain.head.y as f32,
        circle_chain.head.radius as f32,
    ));

    // Draw the circle outlines
    for ch in &circle_chain.circles {
        let radius: f32 = ch.radius as f32;
        let center_x: f32 = ch.x as f32;
        let center_y: f32 = ch.y as f32;
        circles.push((center_x, center_y, radius));
    }
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
