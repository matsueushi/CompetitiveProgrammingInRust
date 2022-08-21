pub mod geom {
    use num::Integer;
    use std::ops::Neg;

    pub fn normalize_line<T>(x0: T, y0: T, x1: T, y1: T) -> (T, T, T)
    where
        T: Copy + Integer + Neg<Output = T>,
    {
        let mut a = y1 - y0;
        let mut b = -(x1 - x0);
        let mut c = x1 * y0 - x0 * y1;
        if a < T::zero() || (a == T::zero() && b < T::zero()) {
            a = -a;
            b = -b;
            c = -c;
        }
        let g = a.gcd(&b);
        (a / g, b / g, c / g)
    }

    pub fn dist2<T>(x0: T, y0: T, x1: T, y1: T) -> T
    where
        T: Copy + Integer,
    {
        let dx = x0 - x1;
        let dy = y0 - y1;
        dx * dx + dy * dy
    }

    pub fn inner_product<T>(ax: T, ay: T, bx: T, by: T, cx: T, cy: T) -> T
    where
        T: Copy + Integer,
    {
        let abx = bx - ax;
        let aby = by - ay;
        let acx = cx - ax;
        let acy = cy - ay;
        abx * acx + aby * acy
    }
}

#[cfg(test)]
mod tests {
    use super::geom::*;

    #[test]
    fn test_normalize_line() {
        assert_eq!(normalize_line(1, 1, 2, 2), (1, -1, 0));
        assert_eq!(normalize_line(1, 0, 2, 0), (0, 1, 0));
        assert_eq!(normalize_line(0, 1, 0, 2), (1, 0, 0));
    }

    #[test]
    fn test_dist2() {
        assert_eq!(dist2(1, 1, -1, -1), 8);
    }

    #[test]
    fn test_inner_product() {
        assert_eq!(inner_product(0, 0, 2, -1, 1, 2), 0);
        assert_eq!(inner_product(0, 0, 5, 0, 2, 4), 10);
    }
}
