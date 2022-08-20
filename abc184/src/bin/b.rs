use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        x: usize,
        s: Chars,
    }
    let mut score = x;
    for si in s {
        match si {
            'o' => {
                score += 1;
            }
            'x' => {
                if score >= 1 {
                    score -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", score);
}
