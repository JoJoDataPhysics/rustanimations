use crate::movements::attractor::Attractor;
use crate::shapes::circle::CircleChain;
use bevy::prelude::*;

/// A Bevy component that combines an attractor-based particle system
/// with a chain of connected circles for creating organic animations.
///
/// The attractor controls the movement of particles through space,
/// while the circle chain creates visual structures that follow
/// the particle's path.
#[derive(Component)]
pub struct AnimatedChain {
    /// The attractor system that controls particle movement
    pub attractor: Attractor,
    /// The chain of circles that creates the visual structure
    pub circle_chain: CircleChain,
}
