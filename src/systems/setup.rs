use crate::components::AnimatedChain;
use crate::config::AnimationConfig;
use crate::movements::attractor::{Attractor, Particle};
use crate::shapes::combi_shapes::seven_chain;
use bevy::prelude::*;

pub fn setup_system(mut commands: Commands, config: Res<AnimationConfig>) {
    commands.spawn(Camera2dBundle::default());

    for _ in 0..config.chain_count {
        let particle = Particle::new();
        let attractor = Attractor::new(particle);
        let circle_chain = seven_chain();

        commands.spawn((
            AnimatedChain {
                attractor,
                circle_chain,
            },
            SpatialBundle::default(),
        ));
    }
}
