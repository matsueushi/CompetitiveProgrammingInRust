use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2 :usize,
        b: [[usize; w2]; h2],
    }
    for i in 0..1 << h1 {
        if (i as usize).count_ones() as usize != h2 {
            continue;
        }
        let mut xpos = Vec::new();
        for k in 0..h1 {
            if i >> k & 1 == 1 {
                xpos.push(k);
            }
        }
        for j in 0..1 << w1 {
            if (j as usize).count_ones() as usize != w2 {
                continue;
            }

            let mut ypos = Vec::new();
            for k in 0..w1 {
                if j >> k & 1 == 1 {
                    ypos.push(k);
                }
            }

            let mut ok = true;
            for p in 0..xpos.len() {
                for q in 0..ypos.len() {
                    if a[xpos[p]][ypos[q]] != b[p][q] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
