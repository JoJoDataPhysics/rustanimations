fn calculate_angle_radians(x: f64, y: f64) -> f64 {
    y.atan2(x) // Compute the angle in radians
}

#[derive(Debug)]
pub struct CircularHelper {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub direction: f64,
}

impl Clone for CircularHelper {
    fn clone(&self) -> CircularHelper {
        CircularHelper {
            x: self.x,
            y: self.y,
            radius: self.radius,
            direction: self.direction,
        }
    }
}

impl CircularHelper {
    pub fn allign_to(&mut self, ref_circle: CircularHelper) -> CircularHelper {
        let dx: f64 = self.x - ref_circle.x;
        let dy: f64 = self.y - ref_circle.y;
        let direction = calculate_angle_radians(dx, dy); // Compute the angle in radians
        let both_r = self.radius + ref_circle.radius;
        let tx = both_r * direction.cos();
        let ty = both_r * direction.sin();
        self.x = ref_circle.x + tx;
        self.y = ref_circle.y + ty;
        self.direction = direction;
        CircularHelper {
            x: ref_circle.x + tx,
            y: ref_circle.y + ty,
            radius: self.radius,
            direction,
        }
    }
    pub fn new(x: f64, y: f64, radius: f64, direction: f64) -> CircularHelper {
        CircularHelper {
            x,
            y,
            radius,
            direction,
        }
    }
}

#[derive(Debug)]
pub struct CircleChain {
    pub head: CircularHelper,
    pub circles: Vec<CircularHelper>,
    pub is_visible_circles: bool,
    pub is_visible_contour: bool,
    pub is_visible_centers: bool,
    pub is_visible_contour_dots: bool,
}

impl CircleChain {
    pub fn new(head: &CircularHelper) -> CircleChain {
        CircleChain {
            head: head.clone(),
            circles: Vec::new(),
            is_visible_circles: true,
            is_visible_contour: true,
            is_visible_centers: true,
            is_visible_contour_dots: true,
        }
    }
    pub fn add_circle(&mut self, circle: &CircularHelper) {
        self.circles.push(circle.clone());
    }
    pub fn move_head(&mut self, x: f64, y: f64) {
        self.head.x += x;
        self.head.y += y;
    }

    pub fn allign_circles(&mut self) {
        if self.circles.is_empty() {
            return;
        }
        let mut new_chain_vec = Vec::new();
        let mut new_circle = self.circles[0].clone();
        new_circle.allign_to(self.head.clone());
        new_chain_vec.push(new_circle);
        if self.circles.len() > 1 {
            for i in 1..self.circles.len() {
                let mut new_circle = self.circles[i].clone();
                new_circle.allign_to(new_chain_vec[i - 1].clone());
                new_chain_vec.push(new_circle);
            }
        }
        self.circles = new_chain_vec;
    }
}
