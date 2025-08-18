#[cfg(test)]
mod tests {
    use crate::shapes::polynomial::{
        get_supporting_point, quadratic_interpolate, QuadraticPolynomial,
    };
    use bevy::math::Vec2;

    #[test]
    fn test_polynomial_initialization() {
        let poly = QuadraticPolynomial::new(1.0, 2.0, 3.0);
        assert_eq!(poly.eval(0.0), 3.0); // c term
        assert_eq!(poly.eval(1.0), 6.0); // a + b + c = 1 + 2 + 3
    }

    #[test]
    fn test_polynomial_update() {
        let mut poly = QuadraticPolynomial::new(1.0, 1.0, 1.0);
        let initial_result = poly.eval(1.0);

        poly.update(2.0, 2.0, 2.0);
        let updated_result = poly.eval(1.0);

        assert_ne!(initial_result, updated_result);
        assert_eq!(updated_result, 6.0); // 2 + 2 + 2
    }

    #[test]
    fn test_quadratic_interpolation() {
        let p1 = Vec2::new(0.0, -1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::new(2.0, -1.0);

        let inter_poly = quadratic_interpolate(p1, p2, p3);

        // The polynomial should pass through all three points
        assert!((inter_poly.eval(p1.x) - p1.y).abs() < 0.001);
        assert!((inter_poly.eval(p2.x) - p2.y).abs() < 0.001);
        assert!((inter_poly.eval(p3.x) - p3.y).abs() < 0.001);
    }

    #[test]
    fn test_supporting_point_generation() {
        let p1 = Vec2::new(0.0, 0.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::new(2.0, 0.0);

        let supporting_point = get_supporting_point(p1, p2, p3, 0.5);

        // Supporting point should be finite
        assert!(supporting_point.x.is_finite());
        assert!(supporting_point.y.is_finite());

        // Test different interpolation factors
        let sp1 = get_supporting_point(p1, p2, p3, 0.25);
        let sp2 = get_supporting_point(p1, p2, p3, 0.75);

        assert_ne!(sp1, sp2);
    }

    #[test]
    fn test_supporting_point_bounds() {
        let p1 = Vec2::new(0.0, -1.0);
        let p2 = Vec2::new(1.0, 1.0);
        let p3 = Vec2::new(2.0, -1.0);

        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let point = get_supporting_point(p1, p2, p3, t);

            assert!(point.x.is_finite());
            assert!(point.y.is_finite());
        }
    }
}
