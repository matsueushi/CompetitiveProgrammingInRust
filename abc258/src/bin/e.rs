use itertools_num::ItertoolsNum;
use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: usize,
        ws: [usize; n],
        ks: [usize; q],
    }

    // sum of ws
    let sumws: usize = ws.iter().sum();
    let cycle = x / sumws;
    let offset = x % sumws;

    // println!("{} {} {} {:?} {:?}", n, q, x, ws, ks);
    let cumsum = std::iter::once(0usize)
        .chain(ws.clone().into_iter())
        .chain(ws.into_iter())
        .cumsum::<usize>()
        .collect::<Vec<_>>();
    // println!("{:?}", cumsum);

    let mut pos = HashMap::new();
    let mut ind = 0;

    let mut st = 0;
    let mut en = 0;

    let mut i = 0;
    let mut ns = vec![];
    loop {
        pos.insert(ind, i);
        // println!("{}", ind);

        // searchsorted
        let newind = cumsum
            .binary_search_by(|&y| {
                if y < cumsum[ind] + offset {
                    Less
                } else {
                    Greater
                }
            })
            .unwrap_or_else(|x| x);

        ns.push(cycle * n + newind - ind);

        ind = newind;
        ind = ind % n;
        i += 1;

        if pos.contains_key(&ind) {
            st = pos[&ind];
            en = i;
            break;
        }
    }

    // println!("{:?}", ns);
    // println!("{} {}", st, en);

    for k in ks {
        let y = k - 1;
        let z = if y < st { y } else { st + (y - st) % (en - st) };
        println!("{}", ns[z]);
    }
}
