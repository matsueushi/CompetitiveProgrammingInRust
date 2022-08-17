use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut m = n;
    let mut v = vec![0; 3];
    let mut s = 0;
    let mut d = 0;
    while m > 0 {
        d += 1;
        let r = m % 10;
        v[r % 3] += 1;
        s = (s + r) % 3;
        m /= 10;
    }
    match s {
        0 => println!("0"),
        1 => {
            if v[1] >= 1 && d >= 2 {
                println!("1");
            } else if v[2] >= 2 && d >= 3 {
                println!("2");
            } else {
                println!("-1");
            }
        }
        2 => {
            if v[2] >= 1 && d >= 2 {
                println!("1");
            } else if v[1] >= 2 && d >= 3 {
                println!("2");
            } else {
                println!("-1");
            }
        }
        _ => unreachable!(),
    }
}
