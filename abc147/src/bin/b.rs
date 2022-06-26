use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut hug = 0;
    let s0 = s.as_bytes();
    let t0 = s.chars().rev().collect::<String>();
    let t1 = t0.as_bytes();
    for i in 0..(s.len() / 2) {
        if s0[i] != t1[i] {
            hug += 1;
        }
    }
    println!("{}", hug);
}
