use crate::shapes::circle::CircleChain;
use crate::shapes::circle::CircularNode;

pub fn test_circle() {
    let mut circle = CircularNode::new(2.0, 2.0, 1.0, 0.0);
    let ref_circle = CircularNode::new(0.0, 0.0, 1.0, 0.0);
    println!("test_circle");
    println!("{circle:?}");
    let new_circle = circle.allign_nodes(ref_circle, 1.0);
    println!("{new_circle:?}");
}

pub fn test_circle_chain() {
    println!("test_circle_chain");
    let head = CircularNode::new(0.0, 0.0, 1.0, 0.0);
    let mut circle_chain = CircleChain::new(&head, 1.0);
    let circle1 = CircularNode::new(0.0, 6.0, 1.0, 0.0);
    let circle2 = CircularNode::new(0.0, 9.0, 1.0, 0.0);
    let circle3 = CircularNode::new(4.0, 0.0, 1.0, 0.0);
    circle_chain.add_circle(&circle1);
    circle_chain.add_circle(&circle2);
    circle_chain.add_circle(&circle3);
    println!("\ncurrent state");
    println!("{head:?}");
    for circle in &circle_chain.circles {
        println!("{circle:?}");
    }
    circle_chain.allign_nodes();
    println!("\nafter allign");
    println!("{head:?}");
    for circle in &circle_chain.circles {
        println!("{circle:?}");
    }
    circle_chain.move_head(11.0, 11.0);
    println!("\nafter move_head");
    println!("{0:?}", circle_chain.head);
    circle_chain.allign_nodes();
    for circle in &circle_chain.circles {
        println!("{circle:?}");
    }
}
