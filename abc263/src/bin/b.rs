use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ps: [usize; n-1],
    }
    let mut res = 1;
    let mut ind = n - 2;
    while ps[ind] != 1 {
        res += 1;
        ind = ps[ind] - 2;
    }
    println!("{}", res);
}
