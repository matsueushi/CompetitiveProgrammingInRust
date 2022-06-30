use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut as_: [usize; k],
        ls: [Usize1; q],
    }
    for l in ls {
        if as_[l] == n || (l != k - 1 && as_[l + 1] == as_[l] + 1) {
            continue;
        }
        as_[l] += 1;
    }
    println!(
        "{}",
        as_.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
