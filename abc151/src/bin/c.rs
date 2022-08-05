use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ps: [(Usize1, String); m],
    }
    let mut ac = vec![false; n];
    let mut wa = vec![0; n];
    for (p, s) in ps {
        if ac[p] {
            continue;
        }

        if s == "WA" {
            wa[p] += 1;
        } else {
            ac[p] = true;
        }
    }

    let mut ans = 0;
    let mut penalty = 0;
    for i in 0..n {
        if ac[i] {
            penalty += wa[i];
            ans += 1;
        }
    }
    println!("{} {}", ans, penalty);
}
