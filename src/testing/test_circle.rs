#[cfg(test)]
mod tests {
    use crate::shapes::circle::CircularNode;
    use crate::shapes::combi_shapes::seven_chain;
    use crate::shapes::countour::seven_node_contour;

    #[test]
    fn test_contour_generation() {
        let contour_nodes = seven_node_contour();
        assert!(!contour_nodes.is_empty());
        assert_eq!(contour_nodes.len(), 16); // 1 + 7 + 1 + 7 nodes total

        for contour_node in contour_nodes {
            assert!(contour_node.center_node_index < 7);
        }
    }

    #[test]
    fn test_circle_alignment() {
        let mut circle = CircularNode::new(2.0, 2.0, 1.0, 0.0);
        let ref_circle = CircularNode::new(0.0, 0.0, 1.0, 0.0);
        let original_x = circle.x;
        let original_y = circle.y;

        let new_circle = circle.allign_nodes(&ref_circle, 1.0);

        // The circle's position should have been modified
        assert!(circle.x != original_x || circle.y != original_y);
        assert_eq!(new_circle.radius, circle.radius);

        // The new circle should be at distance 1.0 from ref_circle
        let distance =
            ((new_circle.x - ref_circle.x).powi(2) + (new_circle.y - ref_circle.y).powi(2)).sqrt();
        assert!((distance - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_circle_chain_initialization() {
        let seven_chain = seven_chain();

        assert_eq!(seven_chain.circles.len(), 7);
        assert!(!seven_chain.is_visible_circles); // Default is false
        assert!(seven_chain.is_visible_contour);
        assert!(seven_chain.is_visible_nodes);
        assert!(seven_chain.is_visible_contour_dots);
    }

    #[test]
    fn test_circle_chain_movement() {
        let mut seven_chain = seven_chain();
        let initial_head = seven_chain.circles[0].clone();

        seven_chain.move_head(11.0, 11.0);
        seven_chain.allign_nodes();

        let new_head = &seven_chain.circles[0];
        assert_ne!(initial_head.x, new_head.x);
        assert_ne!(initial_head.y, new_head.y);
    }
}
