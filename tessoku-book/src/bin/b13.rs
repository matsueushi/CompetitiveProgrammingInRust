use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut cumsum = vec![0];
    for i in 0..n {
        cumsum.push(cumsum[i] + a[i]);
    }

    let mut rs = vec![0; n + 1];
    for i in 0..n {
        if i == 0 {
            rs[i] = 0;
        } else {
            rs[i] = rs[i - 1];
        }

        while rs[i] < n && cumsum[rs[i] + 1] - cumsum[i] <= k {
            rs[i] += 1;
        }
    }

    let mut res = 0;
    for i in 0..n {
        res += rs[i] - i;
    }
    println!("{}", res);
}
