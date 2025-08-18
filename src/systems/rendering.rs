use crate::components::AnimatedChain;
use crate::config::AnimationConfig;
use crate::shapes::countour::seven_node_contour;
use crate::shapes::polynomial::get_supporting_point;
use bevy::prelude::*;

pub fn render_skeleton_system(query: Query<&AnimatedChain>, mut gizmos: Gizmos) {
    for chain in &query {
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
    }
}

pub fn render_contour_system(
    query: Query<&AnimatedChain>,
    config: Res<AnimationConfig>,
    mut gizmos: Gizmos,
) {
    for chain in &query {
        if chain.circle_chain.is_visible_contour {
            let contour_nodes = generate_contour_nodes(chain);
            let contour = generate_smooth_contour(&contour_nodes, &config);

            for i in 0..contour.len() {
                gizmos.line_2d(
                    contour[i],
                    contour[(i + 1) % contour.len()],
                    Color::rgb(0.0, 0.0, 0.5),
                );
            }
        }
    }
}

pub fn render_circles_system(
    mut query: Query<(&AnimatedChain, &mut Transform)>,
    mut gizmos: Gizmos,
) {
    for (chain, mut transform) in &mut query {
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

            if chain.circle_chain.is_visible_indizes {
                let index_x = circle.x + radius * angle.cos();
                let index_y = circle.y + radius * angle.sin();
                let index_pos = Vec3::new(index_x, index_y, 0.0);
                gizmos.circle_2d(index_pos.truncate(), 2.0, Color::rgb(0.1, 0.9, 0.1));
            }
        }
    }
}

pub fn render_contour_dots_system(query: Query<&AnimatedChain>, mut gizmos: Gizmos) {
    for chain in &query {
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

                gizmos.circle_2d(Vec2::new(x, y), 1.0, Color::rgb(0.1, 0.9, 0.1));
            }
        }
    }
}

fn generate_contour_nodes(chain: &AnimatedChain) -> Vec<Vec2> {
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
    contour_nodes
}

fn generate_smooth_contour(contour_nodes: &[Vec2], config: &AnimationConfig) -> Vec<Vec2> {
    let mut contour = Vec::new();
    let num_nodes = contour_nodes.len();

    for i in 0..num_nodes {
        contour.push(contour_nodes[i]);

        for &offset in &config.supporting_point_offsets {
            contour.push(get_supporting_point(
                contour_nodes[i],
                contour_nodes[(i + 1) % num_nodes],
                contour_nodes[(i + 2) % num_nodes],
                offset,
            ));
        }
    }
    contour
}
