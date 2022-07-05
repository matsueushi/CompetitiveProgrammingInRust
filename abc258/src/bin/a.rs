use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    let h = 21 + k / 60;
    let k = k % 60;
    println!("{}:{:02}", h, k);
}
