use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut csum = vec![0];
    let mut csum_max = vec![0];
    let mut ccsum = vec![0];
    let mut d = 0;
    for i in 0..n {
        csum.push(csum[i] + a[i]);
        csum_max.push(csum_max[i].max(csum[i + 1]));
        ccsum.push(ccsum[i] + csum[i + 1]);
        d = d.max(ccsum[i] + csum_max[i + 1]);
    }
    println!("{}", d);
}
