use itertools_num::ItertoolsNum;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        ws: [u64; n],
    }
    let mut wts: Vec<u64> = ws.clone();
    wts.sort();
    wts.dedup();

    let mut posc = vec![0; wts.len() + 1];
    for i in 0..n {
        let pos = wts.binary_search(&ws[i]).unwrap();
        if s[i] == '0' {
            posc[pos + 1] += 1;
        } else {
            posc[0] += 1;
            posc[pos + 1] -= 1;
        }
    }
    let res: i64 = posc.iter().cumsum().max().unwrap();
    println!("{}", res);
}
