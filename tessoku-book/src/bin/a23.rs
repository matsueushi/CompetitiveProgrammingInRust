use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let mut coupon = vec![0; m];
    for i in 0..m {
        let mut c = 0;
        for j in 0..n {
            if a[i][j] == 1 {
                c |= 1 << j;
            }
        }
        coupon[i] = c;
    }

    let mut dp = vec![std::usize::MAX / 2; 1 << n];
    dp[0] = 0;
    for i in 0..m {
        for j in 0..1 << n {
            let j2 = j | coupon[i];
            dp[j2] = dp[j2].min(dp[j] + 1);
        }
    }
    let res = dp.last().unwrap();
    if res > &100 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
