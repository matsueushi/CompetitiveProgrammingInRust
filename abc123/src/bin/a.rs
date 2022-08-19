use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [i64; 5],
        k: i64,
    }
    for i in 0..4 {
        for j in i..5 {
            if (a[i] - a[j]).abs() > k {
                println!(":(");
                return;
            }
        }
    }
    println!("Yay!");
}
