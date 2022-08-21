use proconio::{fastout, input};

fn factorial(n: usize) -> usize {
    match n {
        1 => 1,
        x => x * factorial(x - 1),
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!(
        "{}",
        factorial(n).into_iter().map(|x| x.to_string()).join(" ")
    );
}
