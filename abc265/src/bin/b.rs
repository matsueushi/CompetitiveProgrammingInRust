use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a: [usize; n - 1],
        xy: [(Usize1, usize); m],
    }
    let mut bonus = vec![0; n - 1];
    for (x, y) in xy {
        bonus[x] = y;
    }

    let mut pt = t;
    for i in 0..n - 1 {
        pt += bonus[i];
        if pt <= a[i] {
            println!("No");
            return;
        }
        pt -= a[i];
    }
    println!("Yes");
}
