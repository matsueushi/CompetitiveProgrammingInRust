use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        m: u32,
    }
    let mut res = Vec::new();
    for i in 0..1 << m {
        let mut count = 0;
        for j in 0..m {
            if i >> j & 1 == 1 {
                count += 1;
            }
        }
        if count != n {
            continue;
        }

        let mut v = Vec::new();
        for j in 0..m {
            if i >> j & 1 == 1 {
                v.push(j + 1);
            }
        }
        res.push(v);
    }

    res.sort();
    for v in res {
        println!("{}", v.into_iter().map(|x| x.to_string()).join(" "));
    }
}
