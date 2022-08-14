use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [usize; n],
        mut w: [usize; m],
    }

    h.append(&mut w);
    let mut ind = (0..h.len()).collect::<Vec<_>>();
    ind.sort_by_key(|&i| &h[i]);

    let mut sortedh = Vec::new();
    let mut sortedw = Vec::new();
    let mut indw = Vec::new();
    let mut hsize = 0;
    for i in 0..n + m {
        let j = ind[i];
        if j < n {
            // student
            hsize += 1;
            sortedh.push(h[j]);
        } else {
            // teacher
            indw.push(hsize.min(n - 1));
            sortedw.push(h[j]);
        }
    }

    let m = (n + 1) / 2;
    let mut l_pair_cost = vec![0; m];
    let mut r_pair_cost = vec![0; m];

    for i in 0..m - 1 {
        l_pair_cost[(m - 1) - (i + 1)] =
            l_pair_cost[(m - 1) - i] + sortedh[(n - 1) - 2 * i] - sortedh[(n - 1) - (2 * i + 1)];
        r_pair_cost[i + 1] = r_pair_cost[i] + sortedh[2 * i + 1] - sortedh[2 * i];
    }

    let mut res = std::usize::MAX;
    for k in 0..indw.len() {
        let i = indw[k];
        let j = if i % 2 == 0 { i / 2 } else { (i - 1) / 2 };

        let w0 = sortedw[k];
        let h0 = sortedh[2 * j];
        let d = if w0 < h0 { h0 - w0 } else { w0 - h0 };
        res = res.min(d + l_pair_cost[j] + r_pair_cost[j]);
    }
    println!("{}", res);
}
