use bevy::prelude::*;
use rustanimations::{systems::*, AnimationConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AnimationConfig>()
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                update_particles_system,
                update_circle_chains_system,
                render_skeleton_system,
                render_contour_system,
                render_circles_system,
                render_contour_dots_system,
            )
                .chain(),
        )
        .run();
}
