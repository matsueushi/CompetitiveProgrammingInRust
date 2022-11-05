use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }
    let mut con = vec![vec![]; n];
    for (a, b) in ab {
        con[a].push(b);
        con[b].push(a);
    }
    for mut vs in con {
        vs.sort();
        if vs.is_empty() {
            println!("0")
        } else {
            println!(
                "{} {}",
                vs.len(),
                vs.into_iter().map(|x| (x + 1).to_string()).join(" ")
            )
        }
    }
}
