use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut expmap = HashMap::new();
    let mut maxmap = HashMap::new();

    let mut pes = Vec::new();
    for _ in 0..n {
        input! {
            m: usize,
            pe: [(usize, usize); m],
        }
        pes.push(pe);
    }

    // 最大値、それを実現する回数
    for pe in &pes {
        for (p, e) in pe {
            let count = expmap.entry(p).or_insert(0usize);
            if *count == *e {
                maxmap.entry(p).and_modify(|x| {
                    *x += 1;
                });
            } else if *count < *e {
                expmap.insert(p, *e);
                maxmap.insert(p, 1);
            }
        }
    }

    // println!("{:?} {:?}", expmap, maxmap);

    let mut res = 1;
    for pe in &pes {
        for (p, e) in pe {
            // println!("{} {}", p, e);
            if expmap.get(p) == Some(e) && maxmap.get(p) == Some(&1) {
                res += 1;
                break;
            }
        }
    }

    println!("{}", res.min(n));
}
