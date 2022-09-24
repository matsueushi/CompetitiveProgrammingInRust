use proconio::{fastout, input};

fn seven(x: usize) -> bool {
    for base in [10, 8].iter() {
        let mut x = x;
        while x > 0 {
            let y = x % base;
            if y == 7 {
                return false;
            }
            x /= base;
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n : usize,
    }
    let mut res = 0;
    for i in 1..=n {
        if seven(i) {
            res += 1;
        }
    }
    println!("{}", res);
}
