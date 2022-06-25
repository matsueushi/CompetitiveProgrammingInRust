use proconio::input;

fn main() {
    input! {
        r: usize,
        c:usize,
        a:[[i64; 2]; 2]
    }
    let res = a[r - 1][c - 1];
    println!("{}", res);
}
