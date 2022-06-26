use proconio::input;

fn next_permutation<T: PartialOrd>(nums: &mut [T]) -> bool {
    let last_asc = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        None => {
            nums.reverse();
            return false;
        }
        Some(i) => i,
    };
    match nums[last_asc + 1..]
        .iter()
        .rposition(|x| x > &nums[last_asc])
    {
        None => return false,
        Some(i) => {
            nums.swap(last_asc, last_asc + 1 + i);
            nums[last_asc + 1..].reverse();
        }
    }
    true
}

fn passb(xy: &Vec<(usize, usize)>, a: &usize, b: &usize) -> bool {
    for (x, y) in xy {
        if (x == a && y == b) || (x == b && y == a) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        a: [[u64; n]; n],
        m: usize,
        mut xy: [(usize, usize); m],
    }
    for (x, y) in &mut xy {
        *x -= 1;
        *y -= 1;
    }

    let mut order = (0..n).collect::<Vec<_>>();
    let mut res = std::u64::MAX;
    loop {
        let mut t = 0;
        let mut valid = true;
        for j in 0..n {
            if j > 0 && !passb(&xy, &order[j - 1], &order[j]) {
                valid = false;
                break;
            }
            let i = order[j];
            t += a[i][j];
        }
        if valid {
            res = std::cmp::min(res, t);
        }
        if !next_permutation(&mut order) {
            break;
        }
    }
    let res = if res == std::u64::MAX { -1 } else { res as i64 };
    println!("{}", res);
}
