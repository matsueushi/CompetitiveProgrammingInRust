use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};

const L: usize = 1_000_000_000_000;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        ss: [usize; a],
        ts: [usize; b],
        xs: [usize; q],
    }
    for xi in xs {
        let mut d_left_s = L;
        let mut d_right_s = L;
        let mut d_left_t = L;
        let mut d_right_t = L;

        let sind = ss
            .binary_search_by(|&x| if x < xi { Less } else { Greater })
            .unwrap_or_else(|i| i);
        if sind != a && ss[sind] == xi {
            d_left_s = 0;
            d_right_s = 0;
        } else {
            if sind > 0 {
                d_left_s = xi - ss[sind - 1];
            }
            if sind != a {
                d_right_s = ss[sind] - xi;
            }
        }

        let tind = ts
            .binary_search_by(|&x| if x < xi { Less } else { Greater })
            .unwrap_or_else(|i| i);
        if tind != b && ts[tind] == xi {
            d_left_t = 0;
            d_right_t = 0;
        } else {
            if tind > 0 {
                d_left_t = xi - ts[tind - 1];
            }
            if tind != b {
                d_right_t = ts[tind] - xi;
            }
        }

        let d_left = d_left_s.max(d_left_t);
        let d_right = d_right_s.max(d_right_t);
        let d_1 = d_left_s + d_right_t + d_left_s.min(d_right_t);
        let d_2 = d_left_t + d_right_s + d_left_t.min(d_right_s);
        let d = vec![d_left, d_right, d_1, d_2].into_iter().min().unwrap();
        println!("{}", d);
    }
}
