#![allow(unused)]

use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{cmp::Ordering, collections::HashSet};

use proconio::input;

/// 荷物
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cargo {
    w: usize,
    h: usize,
    d: usize,
    this_side: bool,
    fragile: bool,
    state: usize,
}

impl Cargo {
    pub fn new(w: usize, h: usize, d: usize, this_side: bool, fragile: bool) -> Self {
        Self {
            w,
            h,
            d,
            this_side,
            fragile,
            state: 0,
        }
    }

    /// 荷物の容量
    pub fn volume(&self) -> usize {
        self.w * self.h * self.d
    }

    pub fn set_state(&mut self, state: usize) {
        self.state = state
    }

    pub fn is_valid(&self) -> bool {
        if self.this_side && self.state >= 2 {
            return false;
        }
        true
    }

    pub fn xw(&self) -> usize {
        match self.state {
            0 => self.w,
            1 => self.h,
            2 => self.d,
            3 => self.h,
            4 => self.d,
            _ => self.w,
        }
    }

    pub fn xh(&self) -> usize {
        match self.state {
            0 => self.h,
            1 => self.w,
            2 => self.h,
            3 => self.d,
            4 => self.w,
            _ => self.d,
        }
    }

    pub fn xd(&self) -> usize {
        match self.state {
            0 => self.d,
            1 => self.d,
            2 => self.w,
            3 => self.w,
            4 => self.h,
            _ => self.h,
        }
    }

    /// 荷物が占める領域
    pub fn space(&self) -> (usize, usize, usize) {
        (self.xw(), self.xh(), self.xd())
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
#[derive(Debug)]
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
        let (cw, ch, cd) = cargo.space();
        if self.fragile || cw > self.w || ch > self.h {
            return None;
        }
        let mut plates = Vec::new();
        plates.push(Plate {
            x: self.x,
            y: self.y,
            z: self.z + cd,
            w: cw,
            h: ch,
            fragile: cargo.fragile,
        });

        if cw == self.w && ch < self.h {
            plates.push(Plate {
                x: self.x,
                y: self.y + ch,
                z: self.z,
                w: self.w,
                h: self.h - ch,
                fragile: self.fragile,
            });
        } else if cw < self.w && ch == self.h {
            plates.push(Plate {
                x: self.x + cw,
                y: self.y,
                z: self.z,
                w: self.w - ch,
                h: self.h,
                fragile: self.fragile,
            });
        } else if cw < self.w && ch < self.h {
            if (self.w - cw) < (self.h - ch) {
                /// 横に切る
                plates.push(Plate {
                    x: self.x + cw,
                    y: self.y,
                    z: self.z,
                    w: self.w - cw,
                    h: ch,
                    fragile: self.fragile,
                });
                plates.push(Plate {
                    x: self.x,
                    y: self.y + ch,
                    z: self.z,
                    w: cw,
                    h: self.h - ch,
                    fragile: self.fragile,
                })
            } else {
                // 縦に切る
                plates.push(Plate {
                    x: self.x + cw,
                    y: self.y,
                    z: self.z,
                    w: self.w - cw,
                    h: self.h,
                    fragile: self.fragile,
                });
                plates.push(Plate {
                    x: self.x,
                    y: ch,
                    z: self.z,
                    w: self.w,
                    h: self.h - ch,
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
        Some(AllocResult { cost, plates })
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

    /// 高さ
    pub fn heights(&self) -> Vec<Vec<usize>> {
        let mut hs = vec![vec![0; self.h]; self.w];
        for obj in &self.objs {
            for i in 0..self.w {
                for j in 0..self.h {
                    let mut h = &hs[obj.x + i][obj.y + j];
                    h = h.max(&obj.z);
                }
            }
        }
        hs
    }

    pub fn output_heights(&self) {
        let output_dir = Path::new("tools/out");
        create_dir_all(&output_dir);
        let mut w = BufWriter::new(File::create(output_dir.join("height.csv")).unwrap());
        let hs = self.heights();
        for h in hs {
            writeln!(
                w,
                "{}",
                h.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
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
        for cargo in &mut self.cargos {
            let mut cost = std::usize::MAX;
            for i in 0..6 {
                cargo.set_state(i);
                if !cargo.is_valid() {
                    continue;
                }
                for item in &container.objs {
                    eprintln!("{:?}", item);
                    eprintln!("{:?}", cargo);
                    if let Some(AllocResult { cost: c, plates }) = item.allocate(cargo) {
                        eprintln!("result {:?}", plates);
                        if c < cost {
                            cost = c;
                        }
                    }
                }
            }
        }

        // eprintln!("{:?}", container.heights());
        // container.output_heights();
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
        for _ in 0..a {
            cargos.push(Cargo::new(h, w, d, f != "Y", g != "Y"));
        }
    }

    let mut solver = Solver { w, h, b, d, cargos };
    solver.solve();
}

// cargo run < tools/in/0000.txt > tools/out/0000.txt

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn allocate_cargo() {
//         let plate = Plate {
//             x: 0,
//             y: 0,
//             z: 0,
//             w: 100,
//             h: 100,
//             fragile: true,
//         };
//     }
// }
