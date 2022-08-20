use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: usize,
        mut y: usize,
        mut z: usize,
    }
    std::mem::swap(&mut x, &mut y);
    std::mem::swap(&mut x, &mut z);
    println!("{} {} {}", x, y, z);
}
