pub mod lines {
    use num::Integer;

    pub fn normalize_line(x0: i64, y0: i64, x1: i64, y1: i64) -> (i64, i64, i64) {
        let mut a = y1 - y0;
        let mut b = -(x1 - x0);
        let mut c = x1 * y0 - x0 * y1;
        if a < 0 || (a == 0 && b < 0) {
            a *= -1;
            b *= -1;
            c *= -1;
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
