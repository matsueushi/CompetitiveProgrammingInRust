pub mod lines {
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
}

#[cfg(test)]
mod tests {
    use super::lines::*;

    #[test]
    fn test_normalize_line() {
        assert_eq!(normalize_line(1, 1, 2, 2), (1, -1, 0));
        assert_eq!(normalize_line(1, 0, 2, 0), (0, 1, 0));
        assert_eq!(normalize_line(0, 1, 0, 2), (1, 0, 0));
    }
}
