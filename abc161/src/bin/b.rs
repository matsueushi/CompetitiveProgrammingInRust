use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort_by_key(|&w| std::cmp::Reverse(w));
    let asum: usize = a.iter().sum();
    for i in 0..m {
        if a[i] * 4 * m < asum {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
