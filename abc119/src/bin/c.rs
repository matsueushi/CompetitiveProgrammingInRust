use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [i64; 3],
        l: [i64; n],
    }
    let mut v = vec![vec![0], vec![1], vec![2], vec![3]];
    for _ in 0..n - 1 {
        let mut v2 = Vec::new();
        for vi in v {
            for x in 0..4 {
                let mut vv = vi.clone();
                vv.push(x);
                v2.push(vv);
            }
        }
        v = v2;
    }

    let mut res = std::usize::MAX;
    for vi in &v {
        let mut count = vec![0; 4];
        let mut len = vec![0; 4];
        for i in 0..vi.len() {
            count[vi[i]] += 1;
            len[vi[i]] += l[i];
        }

        let mut is_ok = true;
        let mut cost = 0;
        for i in 0..3 {
            if count[i] == 0 {
                is_ok = false;
                break;
            }
            cost += (count[i] - 1) * 10 + (len[i] - t[i]).abs() as usize;
        }
        if is_ok {
            res = res.min(cost);
        }
    }
    println!("{}", res);
}
