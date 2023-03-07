#![allow(unused)]

use std::{cmp::Ordering, collections::HashSet};

use proconio::input;

/// 荷物
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cargo {
    w: usize,
    h: usize,
    d: usize,
    a: usize,
    this_side: bool,
    fragile: bool,
    state: usize,
}

impl Cargo {
    pub fn new(w: usize, h: usize, d: usize, a: usize, this_side: bool, fragile: bool) -> Self {
        Self {
            w,
            h,
            d,
            a,
            this_side,
            fragile,
            state: 0,
        }
    }

    /// 荷物の容量
    pub fn volume(&self) -> usize {
        self.w * self.h * self.d
    }

    /// 荷物を回転させる
    pub fn rotate(&mut self) {
        self.state += 1;
        if self.this_side {
            self.state = self.state % 2;
        } else {
            self.state = self.state % 6;
        }
    }

    /// 荷物が占める領域
    pub fn space(&self) -> (usize, usize, usize) {
        match self.state {
            0 => (self.w, self.h, self.d),
            1 => (self.h, self.w, self.d),
            2 => (self.d, self.h, self.w),
            3 => (self.h, self.d, self.w),
            4 => (self.d, self.w, self.h),
            _ => (self.w, self.d, self.h),
        }
    }
}

impl Ord for Cargo {
    fn cmp(&self, other: &Self) -> Ordering {
        (!self.fragile, self.volume())
            .cmp(&(!other.fragile, other.volume()))
            .reverse()
    }
}

impl PartialOrd for Cargo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 配置した時の結果
struct AllocResult {
    cost: usize,
    plates: Vec<Plate>,
}

/// 宙に浮くプレート
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Plate {
    x: usize,
    y: usize,
    z: usize,
    w: usize,
    h: usize,
    fragile: bool,
}

impl Plate {
    pub fn area(&self) -> usize {
        self.w * self.h
    }

    pub fn allocate(&self, cargo: &Cargo) -> Option<AllocResult> {
        if self.fragile || cargo.w > self.w || cargo.h > self.h {
            return None;
        }
        let mut plates = Vec::new();
        plates.push(Plate {
            x: self.x,
            y: self.y,
            z: self.z + cargo.d,
            w: cargo.w,
            h: cargo.h,
            fragile: cargo.fragile,
        });

        if cargo.w == self.w && cargo.h < self.h {
            plates.push(Plate {
                x: self.x,
                y: self.y + cargo.h,
                z: self.z,
                w: self.w,
                h: self.h - cargo.h,
                fragile: self.fragile,
            });
        } else if cargo.w < self.w && cargo.h == self.h {
            plates.push(Plate {
                x: self.x + cargo.w,
                y: self.y,
                z: self.z,
                w: self.w - cargo.h,
                h: self.h,
                fragile: self.fragile,
            });
        } else if cargo.w < self.w && cargo.h < self.h {
            if (self.w - cargo.w) < (self.h - cargo.h) {
                /// 横に切る
                plates.push(Plate {
                    x: self.x + cargo.w,
                    y: self.y,
                    z: self.z,
                    w: self.w - cargo.w,
                    h: cargo.h,
                    fragile: self.fragile,
                });
                plates.push(Plate {
                    x: self.x,
                    y: self.y + cargo.h,
                    z: self.z,
                    w: cargo.w,
                    h: self.h - cargo.h,
                    fragile: self.fragile,
                })
            } else {
                // 縦に切る
                plates.push(Plate {
                    x: self.x + cargo.w,
                    y: self.y,
                    z: self.z,
                    w: self.w - cargo.w,
                    h: self.h,
                    fragile: self.fragile,
                });
                plates.push(Plate {
                    x: self.x,
                    y: cargo.h,
                    z: self.z,
                    w: self.w,
                    h: self.h - cargo.h,
                    fragile: self.fragile,
                })
            }
        }
        let cost = if plates.is_empty() {
            0
        } else {
            let mut cost = std::usize::MAX;
            for p in &plates {
                cost = cost.min(p.area());
            }
            cost
        };
        Some(AllocResult { cost: 0, plates })
    }
}

/// コンテナ
struct Container {
    w: usize,
    h: usize,
    b: usize,
    d: usize,
    objs: HashSet<Plate>,
}

impl Container {
    pub fn new(w: usize, h: usize, b: usize, d: usize) -> Self {
        let mut objs = HashSet::new();
        objs.insert(Plate {
            x: 0,
            y: 0,
            z: 0,
            w,
            h,
            fragile: false,
        });
        Self { w, h, b, d, objs }
    }
}

struct Solver {
    w: usize,
    h: usize,
    b: usize,
    d: usize,
    cargos: Vec<Cargo>,
}

impl Solver {
    pub fn solve(&mut self) {
        let mut container = Container::new(self.w, self.h, self.b, self.d);
        let mut solution = vec![0; 1];

        self.cargos.sort();
        for cargo in &self.cargos {
            eprintln!("{:?}", cargo);
        }
    }
}

fn main() {
    input! {
        m: usize,
        w: usize,
        h: usize,
        b: usize,
        d: usize,
        cgs: [(usize, usize, usize, usize, String, String); m]
    }

    eprintln!("m={} w={} h={} b={} d={}", m, w, h, b, d);
    eprintln!("{:?}", cgs);

    let mut cargos = Vec::new();
    for (h, w, d, a, f, g) in cgs {
        cargos.push(Cargo::new(h, w, d, a, f == "Y", g == "Y"));
    }

    let mut solver = Solver { w, h, b, d, cargos };
    solver.solve();
}

// cargo run < tools/in/0000.txt > tools/out/0000.txt
