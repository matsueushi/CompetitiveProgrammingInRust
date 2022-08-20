use proconio::{fastout, input, marker::Chars};

fn check(s: &Vec<Vec<char>>, h: usize, w: usize, k: usize, i: usize) -> Option<usize> {
    let i = i | 1 << (h - 1);
    let par = i.count_ones() as usize;
    let mut ns = vec![0; par];

    let mut div = par - 1;
    for j in 0..w {
        let mut c = 0;
        let mut vs = Vec::new();
        for l in 0..h {
            if s[l][j] == '1' {
                c += 1;
            }
            if (i >> l) & 1 == 1 {
                if c > k {
                    return None;
                }
                vs.push(c);
                c = 0;
            }
        }

        for u in 0..par {
            if ns[u] + vs[u] > k {
                std::mem::swap(&mut ns, &mut vs);
                div += 1;
                break;
            } else {
                ns[u] += vs[u];
            }
        }
    }
    Some(div)
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k :usize,
        s: [Chars; h],
    }

    let mut res = std::usize::MAX;
    for i in 0_usize..1 << (h - 1) {
        match check(&s, h, w, k, i) {
            Some(v) => {
                res = res.min(v);
            }
            None => {}
        }
    }

    println!("{}", res);
}
