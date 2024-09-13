use crate::movements::attractor::{Attractor, Particle};

pub fn test_attractor() {
    let particle = Particle::new();
    let mut attractor = Attractor::new(particle);
    println!("test_attractor");
    println!("{attractor:?}");
    for _ in 0..10 {
        attractor.move_particle();
        println!("{attractor:?}");
    }
}
