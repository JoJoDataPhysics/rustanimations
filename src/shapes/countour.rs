use std::f64::consts::PI;

#[derive(Debug)]
pub struct ContourNode {
    pub center_node_index: usize,
    pub angle: f64,
}

pub fn seven_node_contour() -> Vec<ContourNode> {
    let mut contour_nodes = Vec::new();
    contour_nodes.push(ContourNode {
        center_node_index: 0,
        angle: 0.0,
    });
    for i in 0..7 {
        contour_nodes.push(ContourNode {
            center_node_index: i,
            angle: 0.5 * PI,
        });
    }
    contour_nodes.push(ContourNode {
        center_node_index: 6,
        angle: PI,
    });
    for i in 0..7 {
        contour_nodes.push(ContourNode {
            center_node_index: 6 - i,
            angle: -0.5 * PI,
        });
    }

    contour_nodes
}
