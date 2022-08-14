use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        hw: [(Usize1, Usize1); m],
    }

    let mut hcount = vec![0; h];
    let mut wcount = vec![0; w];
    for (x, y) in &hw {
        hcount[*x] += 1;
        wcount[*y] += 1;
    }

    let maxh = hcount.iter().max().unwrap();
    let maxw = wcount.iter().max().unwrap();

    let mut cross_count = 0;
    for (x, y) in &hw {
        if &hcount[*x] == maxh && &wcount[*y] == maxw {
            cross_count += 1;
        }
    }

    let maxh_count = hcount.iter().filter(|x| *x == maxh).count();
    let maxw_count = wcount.iter().filter(|x| *x == maxw).count();
    let res = if cross_count == maxh_count * maxw_count {
        maxh + maxw - 1
    } else {
        maxh + maxw
    };
    println!("{}", res);
}
