mod movements;
mod shapes;
mod testing;

extern crate bevy;
extern crate rand;
use bevy::prelude::*;

use movements::attractor::Attractor;
use movements::attractor::Particle;
use shapes::circle::CircleChain;
use shapes::combi_shapes::seven_chain;
use shapes::countour::seven_node_contour;
use shapes::polynomial::get_supporting_point;
use testing::test_attractor::test_attractor;
use testing::test_circle::{test_circle, test_circle_chain, test_contour};
use testing::test_polynomial::test_polynomial;

fn main() {
    test_circle();
    test_circle_chain();
    test_contour();
    test_polynomial();
    test_attractor();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_circles)
        .run();
}

#[derive(Component)]
struct AnimatedChain {
    attractor: Attractor,
    circle_chain: CircleChain,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for _ in 0..5 {
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

fn animate_circles(
    _time: Res<Time>,
    mut query: Query<(&mut Transform, &mut AnimatedChain)>,
    mut gizmos: Gizmos,
) {
    for (mut transform, mut chain) in &mut query {
        chain.attractor.move_particle();
        let x = chain.attractor.particle.position.x;
        let y = chain.attractor.particle.position.y;
        chain.circle_chain.position_head(x, y);
        chain.circle_chain.allign_nodes();
        if chain.circle_chain.is_visible_sceleton {
            for i in 0..chain.circle_chain.circles.len() - 1 {
                let node = &chain.circle_chain.circles[i];
                let next_node = &chain.circle_chain.circles[i + 1];
                gizmos.line_2d(
                    Vec2::new(node.x, node.y),
                    Vec2::new(next_node.x, next_node.y),
                    Color::rgb(0.9, 0.1, 0.9),
                );
            }
        }
        if chain.circle_chain.is_visible_contour {
            let mut contour_nodes = Vec::new();
            for node in seven_node_contour() {
                let node_index = node.center_node_index;
                let index_angle = chain.circle_chain.circles[node_index].direction;
                let rel_angle = node.angle;
                let angle = index_angle + rel_angle;
                let center_x = chain.circle_chain.circles[node_index].x;
                let center_y = chain.circle_chain.circles[node_index].y;
                let radius = chain.circle_chain.circles[node_index].radius;
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                contour_nodes.push(Vec2::new(x, y));
            }
            let mut contour = Vec::new();
            let num_nodes = contour_nodes.len();
            for i in 0..num_nodes {
                contour.push(contour_nodes[i]);

                contour.push(get_supporting_point(
                    contour_nodes[i],
                    contour_nodes[(i + 1) % num_nodes],
                    contour_nodes[(i + 2) % num_nodes],
                    0.1,
                ));
                contour.push(get_supporting_point(
                    contour_nodes[i],
                    contour_nodes[(i + 1) % num_nodes],
                    contour_nodes[(i + 2) % num_nodes],
                    0.25,
                ));

                contour.push(get_supporting_point(
                    contour_nodes[i],
                    contour_nodes[(i + 1) % num_nodes],
                    contour_nodes[(i + 2) % num_nodes],
                    0.75,
                ));
                contour.push(get_supporting_point(
                    contour_nodes[i],
                    contour_nodes[(i + 1) % num_nodes],
                    contour_nodes[(i + 2) % num_nodes],
                    0.9,
                ));
            }

            for i in 0..contour.len() {
                gizmos.line_2d(
                    contour[i],
                    contour[(i + 1) % contour.len()],
                    Color::rgb(0.0, 0.0, 0.5),
                );
            }
        }
        for circle in &chain.circle_chain.circles {
            transform.translation = Vec3::new(circle.x, circle.y, 0.0);
            let radius = circle.radius;
            let angle = circle.direction;

            if chain.circle_chain.is_visible_circles {
                gizmos.circle_2d(
                    transform.translation.truncate(),
                    radius,
                    Color::rgb(0.5, 0.5, 0.9),
                );
            }

            if chain.circle_chain.is_visible_nodes {
                let center_radius = 1.0;
                gizmos.circle_2d(
                    transform.translation.truncate(),
                    center_radius,
                    Color::rgb(0.9, 0.5, 0.5),
                );
            }

            let index_x = circle.x + radius * angle.cos();
            let index_y = circle.y + radius * angle.sin();
            transform.translation = Vec3::new(index_x, index_y, 0.0);

            if chain.circle_chain.is_visible_indizes {
                let radius = circle.radius;
                let angle = circle.direction;
                let index_x = circle.x + radius * angle.cos();
                let index_y = circle.y + radius * angle.sin();
                transform.translation = Vec3::new(index_x, index_y, 0.0);
                gizmos.circle_2d(
                    transform.translation.truncate(),
                    2.0,
                    Color::rgb(0.1, 0.9, 0.1),
                );
            }
        }
        if chain.circle_chain.is_visible_contour_dots {
            for contour_node in seven_node_contour() {
                let node_index = contour_node.center_node_index;
                let index_angle = chain.circle_chain.circles[node_index].direction;
                let rel_angle = contour_node.angle;
                let angle = index_angle + rel_angle;
                let center_x = chain.circle_chain.circles[node_index].x;
                let center_y = chain.circle_chain.circles[node_index].y;
                let radius = chain.circle_chain.circles[node_index].radius;
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                transform.translation = Vec3::new(x, y, 0.0);
                gizmos.circle_2d(
                    transform.translation.truncate(),
                    1.0,
                    Color::rgb(0.1, 0.9, 0.1),
                );
            }
        }
    }
}
