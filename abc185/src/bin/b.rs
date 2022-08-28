use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        ab: [(usize, usize); m],
    }
    let mut battery = n;
    let mut ct = 0;
    for (a, b) in ab {
        if battery <= a - ct {
            println!("No");
            return;
        }
        battery -= a - ct;
        battery += b - a;
        battery = battery.min(n);
        ct = b;
    }
    if battery <= t - ct {
        println!("No");
        return;
    }
    println!("Yes");
}
