use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        ss: [Chars; h],
    }
    let mut res = 1;
    for i in 1..h {
        let u = x as i64 - i as i64;
        if u >= 0 && ss[u as usize][y] == '.' {
            res += 1;
        } else {
            break;
        }
    }
    for i in 1..h {
        let u = x + i;
        if u < h && ss[u][y] == '.' {
            res += 1;
        } else {
            break;
        }
    }
    for i in 1..w {
        let v = y as i64 - i as i64;
        if v >= 0 && ss[x][v as usize] == '.' {
            res += 1;
        } else {
            break;
        }
    }
    for i in 1..w {
        let v = y + i;
        if v < w && ss[x][v] == '.' {
            res += 1;
        } else {
            break;
        }
    }
    println!("{}", res);
}
