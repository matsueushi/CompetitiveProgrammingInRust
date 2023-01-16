use proconio::{fastout, input};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdReq {
    pub p: Point,
    pub r: usize,
}

#[allow(dead_code)]
fn atcoder_ad(input: &Vec<AdReq>) -> Vec<Rect> {
    let mut res = Vec::new();
    for AdReq { p, r } in input {
        res.push(Rect {
            p1: Point { x: p.x, y: p.y },
            p2: Point {
                x: p.x + 1,
                y: p.y + 1,
            },
        });
    }
    res
}

fn main_submit() {
    input! {
        n: usize,
        xyr: [(usize, usize, usize); n],
    }
    let mut input = Vec::new();
    for (ai, bi, ri) in xyr {
        input.push(AdReq {
            p: Point { x: ai, y: bi },
            r: ri,
        });
    }
    let res = atcoder_ad(&input);
    for rect in res {
        println!("{} {} {} {}", rect.p1.x, rect.p1.y, rect.p2.x, rect.p2.y);
    }
}

#[allow(dead_code)]
fn main_evaluation() {}

#[fastout]
fn main() {
    main_submit();
}

// cat tools/in/0001.txt | cargo run --bin ahc001-a
