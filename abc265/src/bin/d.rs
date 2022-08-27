use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    }

    let mut v = vec![0];
    for i in 0..a.len() {
        v.push(v[i] + a[i]);
    }
    for x in 0..=n - 3 {
        match v.binary_search(&(v[x] + p)) {
            Ok(_) => match v.binary_search(&(v[x] + p + q)) {
                Ok(_) => match v.binary_search(&(v[x] + p + q + r)) {
                    Ok(_) => {
                        println!("Yes");
                        return;
                    }
                    Err(_) => {}
                },
                Err(_) => continue,
            },
            Err(_) => {
                continue;
            }
        }
    }
    println!("No");
}
