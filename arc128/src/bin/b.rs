use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            r: i64,
            g: i64,
            b: i64,
        }
        let mut x = std::i64::MAX;
        if (r - g) % 3 == 0 {
            x = x.min(r.max(g));
        }
        if (g - b) % 3 == 0 {
            x = x.min(g.max(b));
        }
        if (b - r) % 3 == 0 {
            x = x.min(b.max(r));
        }
        println!("{}", if x == std::i64::MAX { -1 } else { x })
    }
}
