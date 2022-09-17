use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    }

    let mut ab = vec![0; n * n];
    let mut cd = vec![0; n * n];
    for i in 0..n {
        for j in 0..n {
            ab[i * n + j] = a[i] + b[j];
            cd[i * n + j] = c[i] + d[j];
        }
    }
    ab.sort();
    cd.sort();
    for x in ab {
        if let Ok(_) = cd.binary_search(&(k - x)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
