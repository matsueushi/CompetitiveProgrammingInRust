use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        ss: [Chars; 10],
    }
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    for i in 0..10 {
        for j in 0..10 {
            if ss[i][j] == '#' {
                c = i + 1;
                d = j + 1;
            }
        }
    }

    for i in (0..10).rev() {
        for j in (0..10).rev() {
            if ss[i][j] == '#' {
                a = i + 1;
                b = j + 1;
            }
        }
    }
    println!("{} {}", a, c);
    println!("{} {}", b, d);
}
