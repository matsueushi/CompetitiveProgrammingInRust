use proconio::input;

fn main() {
    input! {
        a1: i64,
        a2: i64,
        a3: i64,
    }
    let s = if a1 + a2 + a3 >= 22 { "bust" } else { "win" };
    println!("{}", s);
}
