use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        n : usize,
        lr: [(usize, usize); n],
    }
    let mut timecard = vec![0; t + 1];
    for (l, r) in &lr {
        timecard[*l] += 1;
        timecard[*r] -= 1;
    }
    let mut nps = 0;
    for i in 0..t {
        nps += timecard[i];
        println!("{}", nps);
    }
}
