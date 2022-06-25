use proconio::input;

fn main() {
    input! {
        a: usize,
    }
    let res = 1 << a;
    println!("{}", res);
}
