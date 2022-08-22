pub mod geom {
    use num::Integer;
    use std::ops::Neg;

    #[derive(Copy, Clone)]
    pub struct Point<T>(pub T, pub T);

    pub fn normalize_line<T>(p0: Point<T>, p1: Point<T>) -> (T, T, T)
    where
        T: Copy + Integer + Neg<Output = T>,
    {
        let mut a = p1.1 - p0.1;
        let mut b = -(p1.0 - p0.0);
        let mut c = p1.0 * p0.1 - p0.1 * p1.0;

        if a < T::zero() || (a == T::zero() && b < T::zero()) {
            a = -a;
            b = -b;
            c = -c;
        }
        let g = a.gcd(&b);
        (a / g, b / g, c / g)
    }

    pub fn dist2<T>(p0: Point<T>, p1: Point<T>) -> T
    where
        T: Copy + Integer,
    {
        let dx = p0.0 - p1.0;
        let dy = p0.1 - p1.1;
        dx * dx + dy * dy
    }

    pub fn inner_product<T>(a: Point<T>, b: Point<T>, c: Point<T>) -> T
    where
        T: Copy + Integer,
    {
        let abx = b.0 - a.0;
        let aby = b.1 - a.1;
        let acx = c.0 - a.0;
        let acy = c.1 - a.1;
        abx * acx + aby * acy
    }

    pub fn ccw<T>(p0: Point<T>, p1: Point<T>, p2: Point<T>) -> T
    where
        T: Copy + Integer,
    {
        (p1.0 - p0.0) * (p2.1 - p0.1) - (p1.1 - p0.1) * (p2.0 - p0.0)
    }

    pub fn intersect<T>(p0: Point<T>, p1: Point<T>, p2: Point<T>, p3: Point<T>) -> bool
    where
        T: Copy + Integer,
    {
        let t0 = ccw(p0, p1, p2);
        let t1 = ccw(p0, p1, p3);
        let t2 = ccw(p2, p3, p0);
        let t3 = ccw(p2, p3, p1);
        t0 * t1 <= T::zero() && t2 * t3 <= T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::geom::*;

    #[test]
    fn test_normalize_line() {
        assert_eq!(normalize_line(Point(1, 1), Point(2, 2)), (1, -1, 0));
        assert_eq!(normalize_line(Point(1, 0), Point(2, 0)), (0, 1, 0));
        assert_eq!(normalize_line(Point(0, 1), Point(0, 2)), (1, 0, 0));
    }

    #[test]
    fn test_dist2() {
        assert_eq!(dist2(Point(1, 1), Point(-1, -1)), 8);
    }

    #[test]
    fn test_inner_product() {
        assert_eq!(inner_product(Point(0, 0), Point(2, -1), Point(1, 2)), 0);
        assert_eq!(inner_product(Point(0, 0), Point(5, 0), Point(2, 4)), 10);
    }

    #[test]
    fn test_intersect() {
        assert!(intersect(
            Point(0, 0),
            Point(2, 0),
            Point(1, -1),
            Point(1, 1)
        ));
        assert!(intersect(
            Point(0, 0),
            Point(2, 0),
            Point(1, 0),
            Point(1, 1)
        ));
        assert!(!intersect(
            Point(0, 0),
            Point(2, 0),
            Point(1, -1),
            Point(1, -2)
        ));
    }
}
