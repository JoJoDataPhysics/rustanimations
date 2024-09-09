use std::f64::consts::PI;
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
    pub fn allign_nodes(&mut self, ref_circle: &CircularNode, dist: f64) -> CircularNode {
        let dx: f64 = self.x - ref_circle.x;
        let dy: f64 = self.y - ref_circle.y;
        let direction = calculate_angle_radians(dx, dy); // Compute the angle in radians
        let tx = dist * direction.cos();
        let ty = dist * direction.sin();
        self.x = ref_circle.x + tx;
        self.y = ref_circle.y + ty;
        self.direction = PI + direction;

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
    fn update(&mut self, index: usize, new_circle: CircularNode) -> Result<(), String> {
        if index < self.circles.len() {
            self.circles[index] = new_circle;
            Ok(())
        } else {
            Err(format!("Index {} out of bounds", index))
        }
    }

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

    pub fn position_head(&mut self, x: f64, y: f64) {
        let dx = self.circles[0].x - x;
        let dy = self.circles[0].y - y;
        self.circles[0].x = x;
        self.circles[0].y = y;
        self.circles[0].direction = PI + calculate_angle_radians(dx, dy);
    }

    pub fn allign_nodes(&mut self) {
        if self.circles.is_empty() {
            return;
        }

        if self.circles.len() > 1 {
            for i in 1..self.circles.len() {
                let mut new_circle = self.circles[i].clone();
                let prev_circle = self.circles[i - 1].clone();
                new_circle.allign_nodes(&prev_circle, self.distance);
                self.update(i, new_circle).unwrap();
            }
        }
    }
}
