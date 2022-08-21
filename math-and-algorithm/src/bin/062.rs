use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }
    let mut visit: Vec<i64> = vec![-1; n];
    let mut walk = Vec::new();

    let mut pos = 0;
    let mut ind = 0;
    loop {
        walk.push(pos);
        visit[pos] = ind;

        ind += 1;
        pos = a[pos];
        if visit[pos] >= 0 {
            break;
        }
    }

    let x = if k <= walk.len() {
        k
    } else {
        let st = visit[pos] as usize;
        st + (k - st) % (ind as usize - st)
    };
    println!("{}", walk[x] + 1);
}
