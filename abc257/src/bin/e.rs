use proconio::input;

fn main() {
    input! {
        n: i64,
        cs: [i64; 9],
    }

    let mut rep_c = 0;
    let mut max_rep = 0;
    for i in 0..9 {
        let rep = n / cs[i];
        if rep >= max_rep {
            max_rep = rep;
            rep_c = i;
        }
    }

    let mut budget = n - max_rep * cs[rep_c];
    for _ in 0..max_rep {
        let mut c = rep_c;
        for k in (rep_c + 1..9).rev() {
            if budget >= cs[k] - cs[c] {
                budget -= cs[k] - cs[c];
                c = k;
                break;
            }
        }
        print!("{}", c + 1);
    }
    println!();
}
