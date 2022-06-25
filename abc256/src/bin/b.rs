use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [usize; n]
    }
    let mut res = 0;
    let mut pos = [0; 4];
    for x in xs {
        pos[0] += 1;
        for j in (0..4).rev() {
            if j + x >= 4 {
                res += pos[j];
            } else {
                pos[j + x] += pos[j];
            }
            pos[j] = 0;
        }
    }
    println!("{}", res);
}
