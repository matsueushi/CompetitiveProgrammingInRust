use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: usize,
    }
    let mut xs = Vec::new();
    for _ in 0..4 {
        let y = x % 10;
        xs.push(y);
        x /= 10;
    }
    xs.reverse();

    if xs.iter().cloned().collect::<HashSet<_>>().len() == 1 {
        println!("Weak");
        return;
    }
    let mut ok = false;
    for i in 0..3 {
        if (xs[i] + 1) % 10 != xs[i + 1] {
            ok = true;
            break;
        }
    }
    println!("{}", if ok { "Strong" } else { "Weak" });
}
