use std::io::BufReader;

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin));

    input! {
        from &mut source,
        n: usize,
    }

    let mut l = 1;
    let mut r = n;
    for _ in 0..10 {
        let m = (l + r) / 2;
        println!("? {} {} 1 {}", l, m, n);
        input! {
            from &mut source,
            count: usize,
        }

        if count == m - l + 1 {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let x = l;

    l = 1;
    r = n;
    for _ in 0..10 {
        let m = (l + r) / 2;
        println!("? 1 {} {} {}", n, l, m);
        input! {
            from &mut source,
            count: usize,
        }
        if count == m - l + 1 {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let y = l;

    println!("! {} {}", x, y);
}
