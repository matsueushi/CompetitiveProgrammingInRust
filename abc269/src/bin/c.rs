use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }
    let mut pos = Vec::new();
    for i in 0..60 {
        if x >> i & 1 == 1 {
            pos.push(i);
        }
    }
    // println!("{:?}", pos);
    let mut res = Vec::new();
    for i in 0..1 << pos.len() {
        let mut y: i64 = 0;
        for j in 0..pos.len() {
            if i >> j & 1 == 1 {
                y += 1 << pos[j];
            }
        }
        res.push(y);
    }
    res.sort();
    for y in res {
        println!("{}", y);
    }
}
