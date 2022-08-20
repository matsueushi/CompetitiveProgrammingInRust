use proconio::{fastout, input, marker::Chars};

fn palindrome(s: &Vec<char>) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let m = s.len() / 2;
    let res = palindrome(&s) && palindrome(&s[..m].to_vec()) && palindrome(&s[m + 1..].to_vec());
    println!("{}", if res { "Yes" } else { "No" });
}
