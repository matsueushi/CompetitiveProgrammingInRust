use core::cmp::Reverse;
use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        m: usize,
        w: usize,
        h: usize,
        b: usize,
        d: usize,
        cargos: [(usize, usize, usize, usize, String, String); m]
    }

    eprintln!("m={} w={} h={} b={} d={}", m, w, h, b, d);
    eprintln!("{:?}", cargos);

    let mut xhs = vec![0; m];
    let mut xws = vec![0; m];
    let mut xds = vec![0; m];
    let mut xas = vec![0; m];
    let mut xfs = vec![false; m];
    let mut xgs = vec![false; m];

    for (i, (h, w, d, a, f, g)) in enumerate(cargos) {
        xhs[i] = h;
        xws[i] = w;
        xds[i] = d;
        xas[i] = a;
        xfs[i] = f == "Y";
        xgs[i] = g == "Y";
    }

    let mut ods = (0..xws.len()).collect::<Vec<_>>();
    ods.sort_by_key(|&i| Reverse(&xws[i]));

    let mut current_d = 0;
    for i in 0..m {
        if xgs[i] {
            println!("{} {} {} {} {}", ods[i], 0, b, b, current_d);
            current_d += xds[ods[i]];
        }
    }
    for i in 0..m {
        if !xgs[i] {
            println!("{} {} {} {} {}", ods[i], 0, b, b, current_d);
            current_d += xds[ods[i]];
        }
    }
}

// cargo run < tools/in/0000.txt > tools/out/0000.txt
