use crate::shapes::circle::CircularNode;
use crate::shapes::combi_shapes::seven_chain;
pub fn test_circle() {
    let mut circle = CircularNode::new(2.0, 2.0, 1.0, 0.0);
    let ref_circle = CircularNode::new(0.0, 0.0, 1.0, 0.0);
    println!("test_circle");
    println!("{circle:?}");
    let new_circle = circle.allign_nodes(&ref_circle, 1.0);
    println!("{new_circle:?}");
}

pub fn test_circle_chain() {
    let mut seven_chain = seven_chain();
    println!("\ncurrent state");
    println!("{0:?}", seven_chain.circles[0]);
    for circle in &seven_chain.circles {
        println!("{circle:?}");
    }
    seven_chain.allign_nodes();
    println!("\nafter allign");
    println!("{0:?}", seven_chain.circles[0]);
    for circle in &seven_chain.circles {
        println!("{circle:?}");
    }

    seven_chain.move_head(11.0, 11.0);
    println!("\nafter move_head");
    println!("{0:?}", seven_chain.circles[0]);
    seven_chain.allign_nodes();
    for circle in &seven_chain.circles {
        println!("{circle:?}");
    }
}
