use proconio::{fastout, input, marker::Chars};

fn solve(n: usize, s: &Vec<char>) -> bool {
    if n == 2 {
        return s[0] == s[1];
    }
    match (s[0], s[n - 1]) {
        ('A', 'B') => return false,
        _ => return true,
    };
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let res = if solve(n, &s) { "Yes" } else { "No" };
    println!("{}", res);
}
