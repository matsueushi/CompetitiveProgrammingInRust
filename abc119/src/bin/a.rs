use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let y = s[..4].parse::<usize>().unwrap();
    let m = s[5..7].parse::<usize>().unwrap();
    if (y >= 2019 && m >= 5) || y >= 2020 {
        println!("TBD");
    } else {
        println!("Heisei");
    }
}
