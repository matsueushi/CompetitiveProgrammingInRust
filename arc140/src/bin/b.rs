use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut counts = vec![];

    for i in 1..n - 1 {
        if s[i - 1] == 'A' && s[i] == 'R' && s[i + 1] == 'C' {
            let mut l = i - 1;
            let mut r = i + 1;
            while l >= 1 && s[l - 1] == 'A' {
                l -= 1;
            }
            while r + 1 < n && s[r + 1] == 'C' {
                r += 1;
            }
            let x = (i - l).min(r - i);
            counts.push(x);
        }
    }

    let res = (2 * counts.len()).min(counts.iter().sum());
    println!("{}", res);
}
