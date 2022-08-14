use itertools::Itertools;
use proconio::{fastout, input};

fn solve(n: usize, a: &Vec<usize>) -> Option<(usize, usize)> {
    let mut bits = vec![0; 200];
    let size = n.min(8);

    for i in 1..1 << size {
        let mut x = 0;
        for j in 0..size {
            if i >> j & 1 == 1 {
                x += a[j];
                x %= 200;
            }
        }
        if bits[x] == 0 {
            bits[x] = i;
        } else {
            return Some((bits[x], i));
        }
    }
    None
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let res = solve(n, &a);
    match res {
        Some((u, v)) => {
            println!("Yes");
            let x = u.count_ones();
            let y = v.count_ones();
            let mut xv = Vec::new();
            let mut yv = Vec::new();
            for i in 0..8 {
                if u >> i & 1 == 1 {
                    xv.push(i + 1);
                }
                if v >> i & 1 == 1 {
                    yv.push(i + 1);
                }
            }
            println!("{} {}", x, xv.into_iter().map(|x| x.to_string()).join(" "));
            println!("{} {}", y, yv.into_iter().map(|x| x.to_string()).join(" "));
        }
        None => println!("No"),
    }
}
