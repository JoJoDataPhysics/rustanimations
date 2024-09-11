use std::f32::consts::PI;
fn calculate_angle_radians(x: f32, y: f32) -> f32 {
    y.atan2(x) // Compute the angle in radians
}

#[derive(Debug)]
pub struct CircularNode {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub direction: f32,
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
    pub fn allign_nodes(&mut self, ref_circle: &CircularNode, dist: f32) -> CircularNode {
        let dx: f32 = self.x - ref_circle.x;
        let dy: f32 = self.y - ref_circle.y;
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
    pub fn new(x: f32, y: f32, radius: f32, direction: f32) -> CircularNode {
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
    pub distance: f32,
    pub circles: Vec<CircularNode>,
    pub is_visible_circles: bool,
    pub is_visible_contour: bool,
    pub is_visible_nodes: bool,
    pub is_visible_indizes: bool,
    pub is_visible_contour_dots: bool,
    pub is_visible_sceleton: bool,
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

    pub fn new(head: &CircularNode, dist: f32) -> Self {
        CircleChain {
            distance: dist,
            circles: vec![head.clone()],
            is_visible_circles: false,
            is_visible_contour: true,
            is_visible_nodes: true,
            is_visible_contour_dots: false,
            is_visible_indizes: false,
            is_visible_sceleton: true,
        }
    }
    pub fn add_circle(&mut self, circle: &CircularNode) {
        self.circles.push(circle.clone());
    }
    pub fn move_head(&mut self, x: f32, y: f32) {
        self.circles[0].x += x;
        self.circles[0].y += y;
    }

    pub fn position_head(&mut self, x: f32, y: f32) {
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
