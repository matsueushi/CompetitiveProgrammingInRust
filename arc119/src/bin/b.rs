use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let mut spos = Vec::new();
    let mut tpos = Vec::new();
    for i in 0..n {
        if s[i] == '0' {
            spos.push(i);
        }
        if t[i] == '0' {
            tpos.push(i);
        }
    }

    if spos.len() != tpos.len() {
        println!("-1");
        return;
    }
    let mut res = 0;
    for i in 0..spos.len() {
        if spos[i] != tpos[i] {
            res += 1;
        }
    }
    println!("{}", res);
}
