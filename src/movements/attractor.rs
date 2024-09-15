fish: Unknown command: fzf_key_bindings
~/.config/fish/functions/fish_user_key_bindings.fish (line 1): 
fzf_key_bindings
^~~~~~~~~~~~~~~^
from sourcing file ~/.config/fish/functions/fish_user_key_bindings.fish
	called on line 1 of file ~/.local/share/omf/init.fish
from sourcing file ~/.local/share/omf/init.fish
	called on line 7 of file ~/.config/fish/conf.d/omf.fish
from sourcing file ~/.config/fish/conf.d/omf.fish
	called on line 248 of file /usr/share/fish/config.fish
from sourcing file /usr/share/fish/config.fish
	called during startup
use crate::Vec2;
use rand::Rng;

#[derive(Debug)]
pub struct Attractor {
    pub position: Vec2,
    pub particle: Particle,
}
#[derive(Debug, Clone, Copy)]

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    mass: f32,
}

impl Particle {
    pub fn new() -> Self {
        let position = Vec2::new(0.0, 0.0); //random_position(800.0, 600.0);
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
        let position = random_position(400.0, 300.0);
        Attractor { position, particle }
    }

    pub fn move_particle(&mut self) {
        let friction: f32 = 0.03;
        let force_factor: f32 = 0.5;
        let dist = self.position - self.particle.position;
        let norm_dist = dist.normalize();
        let sum_dist = dist.x.abs() + dist.y.abs();
        if sum_dist < 60.0 {
            self.position = random_position(400.0, 300.0);
            println!("New target: {:?}", self.position);
            return;
        }
        let norm_velocity = self.particle.velocity.normalize();
        let acc =
            (force_factor * norm_dist - friction * self.particle.velocity) / self.particle.mass;
        let proj_acc = acc.dot(norm_velocity);
        if proj_acc < 0.0 {
            // damp (breaking) acceleration oposite to the direction of the velocity
            self.particle.velocity += acc - 0.8 * proj_acc * norm_velocity;
        } else {
            self.particle.velocity += acc;
        }
        self.particle.position += self.particle.velocity;
    }
}

fn random_position(screen_width: f32, screen_height: f32) -> Vec2 {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate random values for x and y within the screen bounds
    let x = rng.gen_range(-1.0 * screen_width..screen_width);
    let y = rng.gen_range(-1.0 * screen_height..screen_height);

    Vec2::new(x, y)
}
