use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }
    lr.sort_by_key(|x| x.1);

    let mut res = 0;
    let mut last_time = 0;
    for (l, r) in lr {
        if l < last_time {
            continue;
        }
        res += 1;
        last_time = r;
    }
    println!("{}", res);
}
