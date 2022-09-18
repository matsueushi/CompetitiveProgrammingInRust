use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let m = n / 2;
    let m2 = n - m;

    let mut b = Vec::new();
    let mut b2 = Vec::new();
    for i in 0..1 << m {
        let mut v = 0;
        for j in 0..m {
            if i >> j & 1 == 1 {
                v += a[j];
            }
        }
        b.push(v);
    }
    for i in 0..1 << m2 {
        let mut v = 0;
        for j in 0..m2 {
            if i >> j & 1 == 1 {
                v += a[j + m];
            }
        }
        b2.push(v);
    }
    b.sort();
    b2.sort();

    for bi in b {
        if let Ok(_) = b2.binary_search(&(k - bi)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
