use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
    }
    let mut c = Vec::new();
    let mut a = Vec::new();
    for _ in 0..n {
        input! {
            ci: usize,
            ai: [usize; m],
        }
        c.push(ci);
        a.push(ai);
    }

    let mut mincost = std::usize::MAX;
    for i in 0..1 << n {
        let mut und = vec![0; m];
        let mut cost = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                cost += c[j];
                for k in 0..m {
                    und[k] += a[j][k];
                }
            }
        }
        // check
        let mut ok = true;
        for k in 0..m {
            if und[k] < x {
                ok = false;
                break;
            }
        }
        if ok {
            mincost = mincost.min(cost);
        }
    }
    if mincost == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", mincost);
    }
}
