use crate::shapes::circle::CircleChain;
use crate::shapes::circle::CircularHelper;

pub fn test_circle() {
    let mut circle = CircularHelper::new(2.0, 2.0, 1.0, 0.0);
    let ref_circle = CircularHelper::new(0.0, 0.0, 1.0, 0.0);
    println!("test_circle");
    println!("{circle:?}");
    let new_circle = circle.allign_to(ref_circle);
    println!("{new_circle:?}");
}

pub fn test_circle_chain() {
    println!("test_circle_chain");
    let head = CircularHelper::new(0.0, 0.0, 1.0, 0.0);
    let mut circle_chain = CircleChain::new(&head);
    let circle1 = CircularHelper::new(0.0, 6.0, 1.0, 0.0);
    let circle2 = CircularHelper::new(0.0, 9.0, 1.0, 0.0);
    let circle3 = CircularHelper::new(4.0, 0.0, 1.0, 0.0);
    circle_chain.add_circle(&circle1);
    circle_chain.add_circle(&circle2);
    circle_chain.add_circle(&circle3);
    println!("\ncurrent state");
    println!("{head:?}");
    for circle in &circle_chain.circles {
        println!("{circle:?}");
    }
    circle_chain.allign_circles();
    println!("\nafter allign");
    println!("{head:?}");
    for circle in &circle_chain.circles {
        println!("{circle:?}");
    }
    circle_chain.move_head(11.0, 11.0);
    println!("\nafter move_head");
    println!("{0:?}", circle_chain.head);
    circle_chain.allign_circles();
    for circle in &circle_chain.circles {
        println!("{circle:?}");
    }
}
