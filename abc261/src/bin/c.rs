use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }
    let mut fds = HashMap::new();
    for s in ss {
        match fds.get_mut(&s) {
            Some(i) => {
                println!("{}({})", s, i);
                *i += 1;
            }
            None => {
                println!("{}", s);
                fds.insert(s, 1);
            }
        }
    }
}
