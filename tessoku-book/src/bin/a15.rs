use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut b = a.clone();
    b.sort();
    b.dedup();

    let mut res = Vec::new();
    for ai in a {
        let ind = b.binary_search(&ai).unwrap() + 1;
        res.push(ind);
    }
    for i in 0..res.len() {
        print!("{}", res[i]);
        if i != res.len() - 1 {
            print!(" ");
        }
    }
    println!();
}
