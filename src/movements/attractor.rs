use crate::Vec2;
use rand::Rng;

#[derive(Debug)]
pub struct Attractor {
    position: Vec2,
    particle: Particle,
}
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
}

impl Particle {
    pub fn new() -> Self {
        let position = random_position(800.0, 600.0);
        let velocity = Vec2::new(0.0, 0.0);
        let mass = 1.0;
        Particle {
            position,
            velocity,
            mass,
        }
    }
}

impl Attractor {
    pub fn new(particle: Particle) -> Self {
        let position = random_position(800.0, 600.0);
        Attractor { position, particle }
    }

    pub fn move_particle(&mut self) {
        let friction: f32 = 0.1;
        let force_factor: f32 = 0.1;
        let norm_dist = (self.position - self.particle.position).normalize();
        let acc =
            (force_factor * norm_dist - friction * self.particle.velocity) / self.particle.mass;
        self.particle.velocity += acc;
        self.particle.position += self.particle.velocity;
    }
}

fn random_position(screen_width: f32, screen_height: f32) -> Vec2 {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate random values for x and y within the screen bounds
    let x = rng.gen_range(0.0..screen_width);
    let y = rng.gen_range(0.0..screen_height);

    Vec2::new(x, y)
}

pub fn get_seven_attractors() -> Vec<Attractor> {
    let mut attractors = Vec::new();
    for _ in 0..7 {
        let particle = Particle::new();
        attractors.push(Attractor::new(particle));
    }
    attractors
}
