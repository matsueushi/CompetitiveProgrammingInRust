use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut md: Vec<i64> = vec![0; 200];
    for x in a {
        md[x % 200] += 1;
    }

    let res: i64 = md.iter().map(|x| x * (x - 1) / 2).sum();
    println!("{}", res);
}
