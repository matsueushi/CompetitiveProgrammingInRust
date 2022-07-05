use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        qs: [(usize, usize); q],
    }
    let ss = s.chars().chain(s.chars()).collect::<Vec<_>>();
    let mut pos = 0;
    let mut res = vec![];
    for (t, x) in &qs {
        match t {
            1 => {
                pos = (pos + n - x) % n;
            }
            2 => {
                res.push(ss[pos + x - 1]);
            }
            _ => unreachable!(),
        }
    }
    for c in res {
        println!("{}", c);
    }
}
