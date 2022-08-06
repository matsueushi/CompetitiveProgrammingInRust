use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        cs: [usize; 5]
    }
    let mut cards = vec![0; 14];
    for c in cs {
        cards[c] += 1;
    }
    let mut two = false;
    let mut three = false;
    for i in 1..=13 {
        if cards[i] == 2 {
            two = true;
        } else if cards[i] == 3 {
            three = true;
        }
    }
    if two && three {
        println!("Yes");
    } else {
        println!("No");
    }
}
