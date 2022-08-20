use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        s: Chars
    }
    let mut xs = vec![0; k];
    let mut ys = vec![0; k];

    let mut count = 0;
    let mut rest = 0;
    for i in 0..n {
        if rest > 0 {
            rest -= 1;
        } else if count < k && s[i] == 'o' {
            xs[count] = i;
            count += 1;
            rest = c;
        }
    }

    count = 0;
    rest = 0;
    for i in (0..n).rev() {
        if rest > 0 {
            rest -= 1;
        } else if count < k && s[i] == 'o' {
            ys[count] = i;
            count += 1;
            rest = c;
        }
    }
    ys.reverse();

    for i in 0..k {
        if xs[i] == ys[i] {
            println!("{}", xs[i] + 1);
        }
    }
}
