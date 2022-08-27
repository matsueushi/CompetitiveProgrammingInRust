use proconio::{fastout, input, marker::Chars};

fn solve(g: &Vec<Vec<char>>, h: usize, w: usize) -> Option<(usize, usize)> {
    let (mut px, mut py) = (0, 0);
    let mut ws = vec![vec![false; w]; h];
    loop {
        if ws[px][py] {
            return None;
        }
        ws[px][py] = true;
        match g[px][py] {
            'U' => {
                if px == 0 {
                    return Some((px, py));
                }
                px -= 1;
            }
            'D' => {
                if px == h - 1 {
                    return Some((px, py));
                }
                px += 1;
            }
            'L' => {
                if py == 0 {
                    return Some((px, py));
                }
                py -= 1;
            }
            'R' => {
                if py == w - 1 {
                    return Some((px, py));
                }
                py += 1;
            }
            _ => unreachable!(),
        }
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    }
    match solve(&g, h, w) {
        Some((x, y)) => {
            println!("{} {}", x + 1, y + 1);
        }
        None => {
            println!("-1");
        }
    };
}
