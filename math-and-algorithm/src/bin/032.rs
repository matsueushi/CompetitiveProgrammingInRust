use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let res = match a.iter().find(|&&t| t == x) {
        Some(_) => "Yes",
        None => "No",
    };
    println!("{}", res);
}
