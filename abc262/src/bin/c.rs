use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut same: i64 = 0;
    let mut res: i64 = 0;
    for i in 1..=n {
        if a[i - 1] == i {
            same += 1;
        } else if a[i - 1] > i {
            let j = a[i - 1];
            if a[j - 1] == i {
                res += 1;
            }
        }
    }

    let additional = if same % 2 == 0 {
        (same / 2) * (same - 1)
    } else {
        same * ((same - 1) / 2)
    };
    res += additional;
    println!("{}", res);
}
