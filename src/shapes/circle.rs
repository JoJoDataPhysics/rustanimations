fn calculate_angle_radians(x: f64, y: f64) -> f64 {
    y.atan2(x) // Compute the angle in radians
}

#[derive(Debug)]
pub struct CircularNode {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub direction: f64,
}

impl Clone for CircularNode {
    fn clone(&self) -> CircularNode {
        CircularNode {
            x: self.x,
            y: self.y,
            radius: self.radius,
            direction: self.direction,
        }
    }
}

impl CircularNode {
    pub fn allign_nodes(&mut self, ref_circle: CircularNode, dist: f64) -> CircularNode {
        let dx: f64 = self.x - ref_circle.x;
        let dy: f64 = self.y - ref_circle.y;
        let direction = calculate_angle_radians(dx, dy); // Compute the angle in radians
        let tx = dist * direction.cos();
        let ty = dist * direction.sin();
        self.x = ref_circle.x + tx;
        self.y = ref_circle.y + ty;
        self.direction = direction;
        CircularNode {
            x: ref_circle.x + tx,
            y: ref_circle.y + ty,
            radius: self.radius,
            direction,
        }
    }
    pub fn new(x: f64, y: f64, radius: f64, direction: f64) -> CircularNode {
        CircularNode {
            x,
            y,
            radius,
            direction,
        }
    }
}

#[derive(Debug)]
pub struct CircleChain {
    pub distance: f64,
    pub circles: Vec<CircularNode>,
    pub is_visible_circles: bool,
    pub is_visible_contour: bool,
    pub is_visible_centers: bool,
    pub is_visible_contour_dots: bool,
}

impl CircleChain {
    pub fn new(head: &CircularNode, dist: f64) -> Self {
        CircleChain {
            distance: dist,
            circles: vec![head.clone()],
            is_visible_circles: true,
            is_visible_contour: true,
            is_visible_centers: true,
            is_visible_contour_dots: true,
        }
    }
    pub fn add_circle(&mut self, circle: &CircularNode) {
        self.circles.push(circle.clone());
    }
    pub fn move_head(&mut self, x: f64, y: f64) {
        self.circles[0].x += x;
        self.circles[0].y += y;
    }

    pub fn allign_nodes(&mut self) {
        if self.circles.is_empty() {
            return;
        }
        let mut new_chain_vec: Vec<CircularNode> = Vec::new();
        new_chain_vec.push(self.circles[0].clone());
        if self.circles.len() > 1 {
            for i in 1..self.circles.len() {
                let mut new_circle: CircularNode = self.circles[i].clone();
                new_circle.allign_nodes(new_chain_vec[i - 1].clone(), self.distance);
                new_chain_vec.push(new_circle);
            }
        }
        self.circles = new_chain_vec;
    }
}
