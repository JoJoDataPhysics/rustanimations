use crate::shapes::circle::CircleChain;
use crate::shapes::circle::CircularNode;

pub fn seven_chain() -> CircleChain {
    let head = CircularNode::new(600.0, 550.0, 10.0, 0.0);
    let mut circle_chain = CircleChain::new(&head, 30.0);
    let circle1 = CircularNode::new(460.0, 560.0, 20.0, 0.0);
    let circle2 = CircularNode::new(660.0, 520.0, 25.0, 0.0);
    let circle3 = CircularNode::new(660.0, 450.0, 15.0, 0.0);
    let circle4 = CircularNode::new(660.0, 450.0, 10.0, 0.0);
    let circle5 = CircularNode::new(660.0, 400.0, 5.0, 0.0);
    let circle6 = CircularNode::new(660.0, 400.0, 5.0, 0.0);
    circle_chain.add_circle(&circle1);
    circle_chain.add_circle(&circle2);
    circle_chain.add_circle(&circle3);
    circle_chain.add_circle(&circle4);
    circle_chain.add_circle(&circle5);
    circle_chain.add_circle(&circle6);
    circle_chain.allign_nodes();
    circle_chain
}
