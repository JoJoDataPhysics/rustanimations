use bevy::math::Vec2; // 2D vector

#[derive(Debug)]
pub struct QuadraticPolynomial {
    a: f32,
    b: f32,
    c: f32,
}

impl QuadraticPolynomial {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
    pub fn update(&mut self, a: f32, b: f32, c: f32) {
        self.a = a;
        self.b = b;
        self.c = c;
    }

    pub fn eval(&self, x: f32) -> f32 {
        self.a * x * x + self.b * x + self.c
    }
}

pub fn is_valid_interpolation(p1: Vec2, p2: Vec2, p3: Vec2) -> bool {
    let (x1, x2, x3) = (p1.x, p2.x, p3.x);
    let a_denominator =
        (x1.powi(2) - x2.powi(2)) * (x2 - x3) - (x2.powi(2) - x3.powi(2)) * (x1 - x2);
    a_denominator != 0.0 && (x1 - x2).abs() > 0.0001 && (x2 - x3).abs() > 0.0001
}

pub fn quadratic_interpolate(p1: Vec2, p2: Vec2, p3: Vec2) -> QuadraticPolynomial {
    let x1 = p1.x;
    let x2 = p2.x;
    let x3 = p3.x;
    let y1 = p1.y;
    let y2 = p2.y;
    let y3 = p3.y;

    let a_numerator = (y1 - y2) * (x2 - x3) - (y2 - y3) * (x1 - x2);
    let a_denominator =
        (x1.powi(2) - x2.powi(2)) * (x2 - x3) - (x2.powi(2) - x3.powi(2)) * (x1 - x2);
    let a = a_numerator / a_denominator;
    let b = ((y1 - y2) - a * (x1.powi(2) - x2.powi(2))) / (x1 - x2);
    let c = y1 - a * x1.powi(2) - b * x1;
    QuadraticPolynomial::new(a, b, c)
}

pub fn get_supporting_point(p1: Vec2, p2: Vec2, p3: Vec2, tau: f32) -> Vec2 {
    let t1 = 0.0;
    let t2 = 1.0;
    let t3 = 2.0;
    let x1 = p1.x;
    let x2 = p2.x;
    let x3 = p3.x;
    let y1 = p1.y;
    let y2 = p2.y;
    let y3 = p3.y;
    let (xp1, xp2, xp3) = (Vec2::new(t1, x1), Vec2::new(t2, x2), Vec2::new(t3, x3));
    let (yp1, yp2, yp3) = (Vec2::new(t1, y1), Vec2::new(t2, y2), Vec2::new(t3, y3));
    let mut x_sup = (xp1.y + xp2.y) / 2.0;
    let mut y_sup = (yp1.y + yp2.y) / 2.0;
    if is_valid_interpolation(xp1, xp2, xp3) && is_valid_interpolation(yp1, yp2, yp3) {
        let y_poly = quadratic_interpolate(yp1, yp2, yp3);
        let x_poly = quadratic_interpolate(xp1, xp2, xp3);
        x_sup = x_poly.eval(tau); // x-coordinate of the supporting point
        y_sup = y_poly.eval(tau); // y-coordinate of the supporting point
    }

    Vec2::new(x_sup, y_sup)
}
