use crate::shapes::polynomial::{get_supporting_point, quadratic_interpolate, QuadraticPolynomial};
use bevy::math::Vec2;

pub fn test_polynomial() {
    let mut poly = QuadraticPolynomial::new(1.0, 1.0, 1.0);
    let mut result = poly.eval(1.0);
    println!("test_polynomial");
    println!("{poly:?}");
    println!("{result:?}");
    poly.update(2.0, 2.0, 2.0);
    result = poly.eval(1.0);
    println!("test_polynomial");
    println!("{poly:?}");
    println!("{result:?}");
    let p1 = Vec2::new(0.0, -1.0);
    let p2 = Vec2::new(1.0, 1.0);
    let p3 = Vec2::new(2.0, -1.0);

    let inter_poly = quadratic_interpolate(p1, p2, p3);
    println!("{inter_poly:?}");
    let mut points = Vec::new();
    points.push(p1);
    points.push(p2);
    points.push(p3);
    for i in 0..points.len() {
        let p = get_supporting_point(
            points[i],
            points[(i + 1) % points.len()],
            points[(i + 2) % points.len()],
            0.75,
        );
        println!("{p:?}");
        println!("points[i] (x, y):({}, {})", points[i].x, points[i].y);
        println!(
            "inter_poly at {}:{}",
            points[i].x,
            inter_poly.eval(points[i].x)
        );
    }

    let p = get_supporting_point(p1, p2, p3, 0.25);
    println!("{p:?}");
}
