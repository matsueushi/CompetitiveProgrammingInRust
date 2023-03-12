#![allow(unused)]

use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::fmt::format;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{cmp::Ordering, collections::HashMap};

use itertools::enumerate;
use proconio::input;

/// 荷物
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cargo {
    num: usize,
    w: usize,
    h: usize,
    d: usize,
    this_side: bool,
    fragile: bool,
    state: usize,
}

impl Cargo {
    pub fn new(num: usize, w: usize, h: usize, d: usize, this_side: bool, fragile: bool) -> Self {
        Self {
            num,
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
                w: self.w - cw,
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
                    w: self.w,
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
                    y: self.y + ch,
                    z: self.z,
                    w: cw,
                    h: self.h - ch,
                    fragile: self.fragile,
                })
            }
        }

        let mut cost = std::usize::MAX;
        // let mut area = 0;
        for p in &plates {
            cost = cost.min(p.area());
            // area += p.area();
        }
        // eprintln!("{:?} {:?} {:?}", &self, &cargo, &plates);
        // assert_eq!(area, self.area());

        Some(AllocResult { cost, plates })
    }
}

/// コンテナ
struct Container {
    w: usize,
    h: usize,
    b: usize,
    d: usize,
    objs: HashMap<usize, Plate>,
    max_id: usize,
    turn: usize,
}

impl Container {
    pub fn new(w: usize, h: usize, b: usize, d: usize) -> Self {
        let mut objs = HashMap::new();
        objs.insert(
            0,
            Plate {
                x: 0,
                y: b,
                z: 0,
                w,
                h: h - 2 * b,
                fragile: false,
            },
        );
        objs.insert(
            1,
            Plate {
                x: b,
                y: 0,
                z: 0,
                w: w - 2 * b,
                h: b,
                fragile: false,
            },
        );
        objs.insert(
            2,
            Plate {
                x: b,
                y: h - b,
                z: 0,
                w: w - 2 * b,
                h: b,
                fragile: false,
            },
        );
        Self {
            w,
            h,
            b,
            d,
            objs,
            max_id: 2,
            turn: 0,
        }
    }

    pub fn plate_position(&self, id: usize) -> (usize, usize, usize) {
        (self.objs[&id].x, self.objs[&id].y, self.objs[&id].z)
    }

    pub fn allocate(&mut self, id: usize, plates: Vec<Plate>) {
        self.turn += 1;
        self.objs.remove(&id);
        for plate in plates {
            self.max_id += 1;
            self.objs.insert(self.max_id, plate);
        }
    }

    /// 高さ
    pub fn heights(&self) -> Vec<Vec<usize>> {
        let mut hs = vec![vec![0; self.h]; self.w];
        for (_, obj) in &self.objs {
            // eprintln!("{:?}", obj);
            // eprintln!("{} {} {} {}", obj.x, self.w, obj.y, self.h);
            for i in 0..obj.w {
                for j in 0..obj.h {
                    hs[obj.x + i][obj.y + j] = hs[obj.x + i][obj.y + j].max(obj.z);
                }
            }
        }
        hs
    }

    pub fn output_heights(&self) {
        eprintln!("turn {}, {:?}", &self.turn, &self.objs);
        let output_dir = Path::new("tools/out");
        create_dir_all(&output_dir);
        let file_name = format!("height_{}.csv", self.turn);
        let mut w = BufWriter::new(File::create(output_dir.join(file_name)).unwrap());
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

type Solution = Vec<(usize, usize, usize, usize, usize)>;

impl Solver {
    pub fn solve_for_cargo(&self, mut cargos: &mut Vec<Cargo>) -> Result<Solution, Solution> {
        let mut container = Container::new(self.w, self.h, self.b, self.d);
        let mut solution = Vec::new();
        for cargo in cargos {
            let mut cost = std::usize::MAX;
            let mut found = false;
            let mut best_id = 0;
            let mut best_plates = Vec::new();
            let mut best_state = 0;
            for i in 0..6 {
                cargo.set_state(i);
                if !cargo.is_valid() {
                    continue;
                }
                for (id, item) in &container.objs {
                    if let Some(AllocResult { cost: c, plates }) = item.allocate(cargo) {
                        if c < cost {
                            found = true;
                            cost = c;
                            best_id = *id;
                            best_plates = plates;
                            best_state = i;
                        }
                    }
                }
            }

            if found {
                cargo.set_state(best_state);
                let (x, y, z) = container.plate_position(best_id);
                solution.push((cargo.num, best_state, x, y, z));
                container.allocate(best_id, best_plates);
            } else {
                // eprintln!("error: {:?}", cargo);
                return Err(solution);
            }
            // container.output_heights();
        }
        Ok(solution)
    }

    pub fn solve(&self) -> Solution {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
        let mut cargos = self.cargos.clone();
        let mut sol = Vec::new();
        for _ in 0..100 {
            match self.solve_for_cargo(&mut cargos) {
                Ok(solution) => return solution,
                Err(solution) => {
                    // eprintln!("Solution not found");
                    sol = solution;
                }
            }
            cargos.shuffle(&mut rng);
        }
        sol // 暫定解を返す
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

    // eprintln!("m={} w={} h={} b={} d={}", m, w, h, b, d);
    // eprintln!("{:?}", cgs);

    let mut cargos = Vec::new();
    for (num, (h, w, d, a, f, g)) in enumerate(cgs) {
        for _ in 0..a {
            cargos.push(Cargo::new(num, w, h, d, f != "Y", g != "Y"));
        }
    }

    let mut solver = Solver { w, h, b, d, cargos };
    let solution = solver.solve();
    for (p, r, x, y, z) in solution {
        println!("{} {} {} {} {}", p, r, x, y, z);
    }
}

// cargo run < tools/in/0000.txt > tools/out/0000.txt
