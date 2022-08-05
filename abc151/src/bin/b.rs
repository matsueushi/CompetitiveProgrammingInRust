use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n-1],
    }
    let need: i64 = (n * m) as i64 - a.iter().sum::<usize>() as i64;
    if need <= 0 {
        println!("0");
    } else if need > k as i64 {
        println!("-1");
    } else {
        println!("{}", need);
    }
}
