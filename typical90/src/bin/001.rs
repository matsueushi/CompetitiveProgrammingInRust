use proconio::input;

fn divisible(as_: &Vec<usize>, l: usize, k: usize, s: usize) -> bool {
    let mut n = 0;
    let mut pos = 0;
    for &a in as_ {
        if a - pos >= s {
            pos = a;
            n += 1;
            if n == k {
                break;
            }
        }
    }
    if n < k {
        false
    } else {
        l - pos >= s
    }
}

fn solve(l: usize, k: usize, as_: &Vec<usize>) -> usize {
    let mut lb = 0;
    let mut ub = l / (k + 1) + 1;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if divisible(&as_, l, k, m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    lb
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        as_: [usize; n],
    }
    let res = solve(l, k, &as_);
    println!("{}", res);
}
