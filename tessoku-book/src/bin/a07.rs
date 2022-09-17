use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(Usize1, Usize1); n],
    }
    let mut diff = vec![0; d + 1];
    for (l, r) in &lr {
        diff[*l] += 1;
        diff[*r + 1] -= 1;
    }
    let mut att = vec![0];
    for i in 0..d {
        att.push(att[i] + diff[i]);
    }
    for i in 1..=d {
        println!("{}", att[i]);
    }
}
