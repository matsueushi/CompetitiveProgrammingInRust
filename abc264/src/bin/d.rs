use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }

    let a = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let mut count = 0;
    for i in (0..7).rev() {
        for j in 0..i {
            if s[j] == a[i] {
                count += 1;
                s.swap(j, j + 1);
            }
        }
    }
    println!("{}", count);
}
