use num::integer::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
    }
    let mut used = vec![false; 100 + t + 1];
    for i in 1..=100 {
        // let x = ((100.0 + t as f64) / 100.0 * (i as f64)).floor() as usize;
        let x = (i * (100 + t)).div_floor(&100);
        used[x] = true;
    }
    let mut skip = Vec::new();
    for i in 1..=100 + t {
        if !used[i] {
            skip.push(i);
        }
    }
    let mut x = n / t;
    let mut y = n % t;
    if y == 0 {
        x -= 1;
        y += t;
    }
    println!("{}", skip[y - 1] + x * (100 + t));
}
