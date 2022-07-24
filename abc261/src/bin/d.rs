use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        mut cy: [(usize, usize); m],
    }
    cy.sort_unstable_by_key(|&(i, _)| i);
    let mut state = vec![0; n + 1];
    state[0] += x[0];
    match cy[0] {
        (1, y) => state[0] += y,
        (_, _) => {}
    }

    for i in 1..n {
        let mut prev = vec![0; n + 1];
        std::mem::swap(&mut state, &mut prev);

        for j in 0..=i {
            state[j] += prev[j] + x[i];
            state[i + 1] = state[i + 1].max(prev[j]);
        }

        // bonus
        for (c, y) in &cy {
            if c <= &(i + 1) {
                state[i + 1 - c] += y;
            }
        }
    }
    let res = state.iter().max().unwrap();
    println!("{}", res);
}
