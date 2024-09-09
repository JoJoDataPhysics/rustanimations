mod movements;
mod shapes;
mod testing;

extern crate bevy;
extern crate rand;
use bevy::prelude::*;
use rand::Rng;

use shapes::circle::CircleChain;
use shapes::combi_shapes::seven_chain;
use testing::test_circle::{test_circle, test_circle_chain};

fn main() {
    test_circle();
    test_circle_chain();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_circles)
        .run();
}

#[derive(Component)]
struct AnimatedChain {
    speed: f32,
    orbit_radius: f32,
    orbit_center: Vec2,
    circle_chain: CircleChain,
    angle: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let orbit_radius = rng.gen_range(150.0..300.0);
        let speed = rng.gen_range(1.0..3.0);
        let orbit_center = Vec2::new(rng.gen_range(-200.0..200.0), rng.gen_range(-200.0..200.0));
        let circle_chain = seven_chain();

        commands.spawn((
            AnimatedChain {
                speed,
                orbit_radius,
                orbit_center,
                circle_chain,
                angle: rng.gen_range(0.0..std::f32::consts::TAU),
            },
            SpatialBundle::default(),
        ));
    }
}

fn animate_circles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut AnimatedChain)>,
    mut gizmos: Gizmos,
) {
    for (mut transform, mut chain) in &mut query {
        chain.angle += chain.speed * time.delta_seconds();

        let x = 1.5 * chain.orbit_center.x + chain.orbit_radius * chain.angle.cos();
        let y = chain.orbit_center.y + chain.orbit_radius * chain.angle.sin();

        chain.circle_chain.position_head(x as f64, y as f64);
        chain.circle_chain.allign_nodes();
        for circle in &chain.circle_chain.circles {
            transform.translation = Vec3::new(circle.x as f32, circle.y as f32, 0.0);
            let radius = circle.radius as f32;
            let angle = circle.direction as f32;
            gizmos.circle_2d(
                transform.translation.truncate(),
                radius,
                Color::rgb(0.4, 0.4, 1.0),
            );

            gizmos.circle_2d(
                transform.translation.truncate(),
                2.0,
                Color::rgb(0.9, 0.1, 0.1),
            );

            let index_x = circle.x as f32 + radius * angle.cos();
            let index_y = circle.y as f32 + radius * angle.sin();
            transform.translation = Vec3::new(index_x, index_y, 0.0);
            gizmos.circle_2d(
                transform.translation.truncate(),
                2.0,
                Color::rgb(0.1, 0.9, 0.1),
            );
        }
    }
}
