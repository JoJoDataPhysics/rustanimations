#[cfg(test)]
mod tests {
    use crate::movements::attractor::{Attractor, Particle};

    #[test]
    fn test_attractor_initialization() {
        let particle = Particle::new();
        let attractor = Attractor::new(particle);

        assert!(attractor.particle.position.x.is_finite());
        assert!(attractor.particle.position.y.is_finite());
        assert!(attractor.particle.velocity.x.is_finite());
        assert!(attractor.particle.velocity.y.is_finite());
    }

    #[test]
    fn test_particle_movement() {
        let particle = Particle::new();
        let mut attractor = Attractor::new(particle);
        let initial_position = attractor.particle.position;

        for _ in 0..10 {
            attractor.move_particle();
        }

        let final_position = attractor.particle.position;
        assert!(
            (initial_position.x - final_position.x).abs() > 0.01
                || (initial_position.y - final_position.y).abs() > 0.01,
            "Particle should have moved from its initial position"
        );
    }

    #[test]
    fn test_particle_bounds() {
        let particle = Particle::new();
        let mut attractor = Attractor::new(particle);

        for _ in 0..1000 {
            attractor.move_particle();

            assert!(attractor.particle.position.x.is_finite());
            assert!(attractor.particle.position.y.is_finite());
            assert!(attractor.particle.velocity.x.is_finite());
            assert!(attractor.particle.velocity.y.is_finite());
        }
    }
}
