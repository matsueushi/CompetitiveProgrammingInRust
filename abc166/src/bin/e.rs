use proconio::{fastout, input};
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut c = Vec::new();
    let mut cmap = HashMap::new();
    for i in 0..n {
        c.push(a[i] - i as i64);
        cmap.entry(c[i]).or_insert(Vec::new()).push(i);
    }
    let mut res = 0;
    for i in 0..n {
        let b = -(a[i] + i as i64);
        if let Some(v) = cmap.get(&b) {
            let i = v
                .binary_search_by(|&x| if !(x >= i) { Less } else { Greater })
                .unwrap_or_else(|i| i);
            res += v.len() - i;
        }
    }
    println!("{}", res);
}
