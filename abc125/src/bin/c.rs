use proconio::{fastout, input};

fn factors(n: usize) -> Vec<usize> {
    let mut fs = Vec::new();
    let mut i = 1;
    while i * i <= n {
        fs.push(i);
        if i != n / i {
            fs.push(n / i);
        }
        i += 1;
    }
    fs
}

fn solve(a: &Vec<usize>, i: usize) -> usize {
    let fcs = factors(a[i]);
    let mut res = 1;
    for f in fcs {
        let mut indivisible = 0;
        for j in 0..a.len() {
            if a[j] % f != 0 {
                indivisible += 1;
            }
            if indivisible > 1 {
                break;
            }
        }
        if indivisible <= 1 {
            res = res.max(f);
        }
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut res = 1;
    res = res.max(solve(&a, 0));
    res = res.max(solve(&a, 1));
    println!("{}", res);
}
