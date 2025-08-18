use crate::components::AnimatedChain;
use bevy::prelude::*;

pub fn update_particles_system(mut query: Query<&mut AnimatedChain>) {
    for mut chain in &mut query {
        chain.attractor.move_particle();
    }
}

pub fn update_circle_chains_system(mut query: Query<&mut AnimatedChain>) {
    for mut chain in &mut query {
        let x = chain.attractor.particle.position.x;
        let y = chain.attractor.particle.position.y;
        chain.circle_chain.position_head(x, y);
        chain.circle_chain.allign_nodes();
    }
}
