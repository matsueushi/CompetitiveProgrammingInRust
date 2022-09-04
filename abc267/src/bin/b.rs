use proconio::{fastout, input, marker::Chars};

fn solve(pins: &Vec<char>) -> bool {
    if pins[0] == '1' {
        return false;
    }
    let pos = [3, 2, 4, 1, 3, 5, 0, 2, 4, 6];
    let mut count = vec![1, 1, 2, 2, 2, 1, 1];
    for i in 0..10 {
        if pins[i] == '0' {
            count[pos[i]] -= 1;
        }
    }

    for i in 0..=4 {
        if count[i] == 0 {
            continue;
        }
        for j in i + 2..=6 {
            if count[j] == 0 {
                continue;
            }
            for k in i + 1..=j - 1 {
                if count[k] == 0 {
                    return true;
                }
            }
        }
    }
    false
}

#[fastout]
fn main() {
    input! {
       pins: Chars,
    }
    println!("{}", if solve(&pins) { "Yes" } else { "No" });
}
