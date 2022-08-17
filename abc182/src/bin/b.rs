use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut gcd = 0;
    let mut res = 0;
    for k in 2..=1000 {
        let mut c = 0;
        for x in &a {
            if x % k == 0 {
                c += 1;
            }
        }
        if c >= res {
            res = c;
            gcd = k;
        }
    }
    println!("{}", gcd);
}
