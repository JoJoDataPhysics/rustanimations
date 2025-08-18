use bevy::prelude::*;

/// Configuration resource for the animation system.
///
/// This resource controls various aspects of the animation including
/// the number of animated chains, smoothness of contours, and
/// interpolation parameters for curve generation.
#[derive(Resource)]
pub struct AnimationConfig {
    /// Number of animated chains to spawn
    pub chain_count: usize,
    /// Smoothness factor for contour generation (lower = smoother)
    pub contour_smoothness: f32,
    /// Offsets for generating supporting points in curve interpolation
    pub supporting_point_offsets: [f32; 4],
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            chain_count: 9,
            contour_smoothness: 0.1,
            supporting_point_offsets: [0.1, 0.25, 0.75, 0.9],
        }
    }
}
