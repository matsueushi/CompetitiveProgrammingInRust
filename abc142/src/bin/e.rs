use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..m {
        input! {
            ai: usize,
            bi: usize,
            ci: [usize; bi],
        }
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    let mm = std::usize::MAX >> 1;

    let mut v2 = vec![mm; 1 << n];
    v2[0] = 0;
    for i in 0..m {
        let v1 = v2.clone();

        let mut mask = 0;
        for x in &c[i] {
            mask |= 1 << (x - 1);
        }

        for j in 0..1 << n {
            v2[j | mask] = v2[j | mask].min(v1[j] + a[i]);
        }
    }

    if v2[(1 << n) - 1] == mm {
        println!("-1");
    } else {
        println!("{}", v2[(1 << n) - 1]);
    }
}
