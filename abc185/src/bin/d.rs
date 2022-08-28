use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    a.push(0);
    a.push(n + 1);
    a.sort();
    let d: Vec<usize> = a
        .windows(2)
        .map(|w| w[1] - w[0] - 1)
        .filter(|x| *x > 0)
        .collect();
    if d.is_empty() {
        println!("0");
        return;
    }
    let w = d.iter().min().unwrap();
    let res = d
        .iter()
        .map(|x| num::Integer::div_ceil(x, &w))
        .sum::<usize>();
    println!("{}", res);
}
