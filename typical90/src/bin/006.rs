use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut pos = 0;
    let mut res = Vec::new();
    for i in 0..k {
        let b = &s[pos..=n - (k - i)]
            .iter()
            .enumerate()
            .min_by_key(|&(_, x)| x)
            .unwrap();
        pos += b.0 + 1;
        res.push(b.1);
    }
    println!("{}", res.into_iter().collect::<String>());
}
