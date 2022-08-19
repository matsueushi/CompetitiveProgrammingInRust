use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        b: String,
    }
    if b == "A" {
        println!("T");
    } else if b == "T" {
        println!("A");
    } else if b == "C" {
        println!("G");
    } else {
        println!("C");
    }
}
