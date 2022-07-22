use proconio::{fastout, input};

#[fastout]
fn main() {
    // matrix
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }

    let mut ah = vec![0; w];
    let mut aw = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            ah[j] += a[i][j];
            aw[i] += a[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", ah[j] + aw[i] - a[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
