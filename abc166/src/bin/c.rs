use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        ab: [(Usize1, Usize1); m],
    }
    let mut height = vec![0; n];
    for (a, b) in ab {
        height[a] = height[a].max(h[b]);
        height[b] = height[b].max(h[a]);
    }
    let mut res = 0;
    for i in 0..n {
        if height[i] < h[i] {
            res += 1;
        }
    }
    println!("{}", res);
}
