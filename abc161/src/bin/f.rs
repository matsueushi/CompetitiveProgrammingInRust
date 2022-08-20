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
    let divs = divisor(n);
    let divs2 = divisor(n - 1);
    let mut res = divs2.len() - 1;
    for d in &divs {
        if *d == 1 {
            continue;
        }
        let mut x = n;
        while x >= *d {
            if x % d == 0 {
                x /= d;
            } else {
                x %= d;
            }
        }

        if x == 1 {
            res += 1;
        }
    }

    println!("{}", res);
}
