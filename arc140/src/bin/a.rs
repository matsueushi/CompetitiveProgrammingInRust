use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    for i in 1..=n {
        if n % i != 0 {
            continue;
        }
        let r = n / i;
        let mut chg = 0;
        for j in 0..i {
            let mut cs = vec![0; 26];
            for x in 0..r {
                let pos = j + x * i;
                let ind = s[pos] as usize - 'a' as usize;
                cs[ind] += 1;
            }
            chg += cs.iter().sum::<usize>() - cs.iter().max().unwrap();
        }
        if chg <= k {
            println!("{}", i);
            break;
        }
    }
}
