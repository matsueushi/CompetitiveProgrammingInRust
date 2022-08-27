use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    if h == 1 || w == 1 {
        println!("1");
        return;
    }
    let c = w / 2;
    let res = h * c + (w % 2) * ((h + 1) / 2);
    println!("{}", res);
}
