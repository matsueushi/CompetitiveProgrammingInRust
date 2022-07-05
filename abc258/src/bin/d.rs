use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    let mut res = std::usize::MAX;
    let mut minplay = std::usize::MAX;
    let mut prepare = 0;
    let mut m = x;
    for (a, b) in &ab {
        m -= 1;
        minplay = std::cmp::min(minplay, *b);
        prepare += *a + *b;
        res = std::cmp::min(res, prepare + m * minplay);
        if m == 0 {
            break;
        }
    }
    println!("{}", res);
}
