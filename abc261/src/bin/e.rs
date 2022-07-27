use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        ta: [(usize, usize); n],
    }
    let mut res = vec![0; n];

    // 各ビット毎
    for i in 0..30 {
        let mut fx = c >> i & 1usize;
        let mut fc = (0, 1);
        for k in 0..n {
            let (t, a) = &ta[k];
            let x = a >> i & 1usize;
            fc = match t {
                1 => (fc.0 & x, fc.1 & x),
                2 => (fc.0 | x, fc.1 | x),
                3 => (fc.0 ^ x, fc.1 ^ x),
                _ => unreachable!(),
            };
            fx = if fx == 0 { fc.0 } else { fc.1 };
            res[k] |= fx << i;
        }
    }

    for x in res {
        println!("{}", x);
    }
}
