use proconio::{fastout, input};

pub fn divisor(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ds = divisor(n);
    let mut res = std::usize::MAX;
    for x in ds {
        res = res.min(2 * x + 2 * (n / x));
    }
    println!("{}", res);
}
