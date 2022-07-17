use proconio::{fastout, input, marker::Chars};

fn normalize(s: &Vec<char>) -> (Vec<char>, Vec<usize>) {
    let mut chars = vec![];
    let mut size = vec![];
    for i in 0..s.len() {
        if i > 0 && s[i] == s[i - 1] {
            if let Some(last) = size.last_mut() {
                *last += 1;
            }
        } else {
            chars.push(s[i]);
            size.push(1);
        }
    }
    (chars, size)
}

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let (schars, ssize) = normalize(&s);
    let (tchars, tsize) = normalize(&t);
    if schars != tchars {
        println!("No");
        return;
    }
    for i in 0..schars.len() {
        if (ssize[i] == 1 && tsize[i] > 1) || (ssize[i] > tsize[i]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
