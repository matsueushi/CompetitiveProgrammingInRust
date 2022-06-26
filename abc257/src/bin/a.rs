use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
    }
    let m = ((x - 1) / n) as u8;
    let c = 'A' as u8 + m;
    println!("{}", c as char);
}
