use proconio::{fastout, input, marker::Chars};

fn solve(n: usize, a: &Vec<Vec<char>>) -> bool {
    for i in 1..n {
        for j in 0..i {
            match (&a[i][j], &a[j][i]) {
                (&'W', &'L') => {}
                (&'L', &'W') => {}
                (&'D', &'D') => {}
                (_, _) => return false,
            }
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let res = solve(n, &a);
    println!("{}", if res { "correct" } else { "incorrect" });
}
