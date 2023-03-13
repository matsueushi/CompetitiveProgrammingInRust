#![allow(unused)]

use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::cmp::Reverse;
use std::fmt;
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
    p: usize,
    w: usize,
    h: usize,
    d: usize,
    this_side: bool,
    fragile: bool,
    state: usize,
}

impl Cargo {
    pub fn new(p: usize, w: usize, h: usize, d: usize, this_side: bool, fragile: bool) -> Self {
        Self {
            p,
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
    depth: usize,
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

        Some(AllocResult {
            depth: self.z + cd,
            cost,
            plates,
        })
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
    pub fn new(w: usize, h: usize, b: usize, d: usize, seed: usize) -> Self {
        let mut objs = HashMap::new();
        if seed % 2 == 0 {
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
        } else {
            objs.insert(
                0,
                Plate {
                    x: b,
                    y: 0,
                    z: 0,
                    w: w - 2 * b,
                    h,
                    fragile: false,
                },
            );
            objs.insert(
                1,
                Plate {
                    x: 0,
                    y: b,
                    z: 0,
                    w: b,
                    h: h - 2 * b,
                    fragile: false,
                },
            );
            objs.insert(
                2,
                Plate {
                    x: w - b,
                    y: b,
                    z: 0,
                    w: b,
                    h: h - 2 * b,
                    fragile: false,
                },
            );
        }

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

type Pos = (usize, usize, usize);

struct Solution {
    positions: Vec<Pos>,
    cargos: Vec<Cargo>,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            cargos: Vec::new(),
        }
    }

    pub fn add(&mut self, pos: Pos, cargo: Cargo) {
        self.positions.push(pos);
        self.cargos.push(cargo);
    }

    pub fn penalty(&self, d: usize) -> usize {
        let mut h = 0;
        let mut overflow_vol = 0;
        for i in 0..self.positions.len() {
            let nh = self.positions[i].2 + self.cargos[i].d;
            h = h.max(nh);
            if nh > d {
                overflow_vol += self.cargos[i].volume();
            }
        }
        // 正確ではない
        let mut p = 1000 + h;
        if h > d {
            p += 1_000_000 + 1000 * overflow_vol;
        }
        p
    }

    pub fn print_result(&self) {
        for i in 0..self.positions.len() {
            let (x, y, z) = self.positions[i];
            println!(
                "{} {} {} {} {}",
                self.cargos[i].p, self.cargos[i].state, x, y, z
            )
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
    pub fn solve_for_cargo(
        &self,
        mut cargos: &mut Vec<Cargo>,
        seed: usize,
    ) -> Result<Solution, Solution> {
        let mut container = Container::new(self.w, self.h, self.b, self.d, seed);
        let mut solution = Solution::new();
        for cargo in cargos {
            let mut found = false;
            let mut max_depth = std::usize::MAX;
            let mut cost = std::usize::MAX;
            let mut best_id = 0;
            let mut best_plates = Vec::new();
            let mut best_state = 0;
            for i in 0..6 {
                cargo.set_state(i);
                if !cargo.is_valid() {
                    continue;
                }
                for (id, item) in &container.objs {
                    if let Some(AllocResult {
                        depth,
                        cost: c,
                        plates,
                    }) = item.allocate(cargo)
                    {
                        // 高さがdより大きい→コストを見る
                        // 高さがdより小さい→最高の高さがd以上またはコストを更新していたらok
                        if depth < max_depth {
                            found = true;
                            max_depth = max_depth.min(depth);
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
                let pos = container.plate_position(best_id);
                solution.add(pos, *cargo);
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
        // 1.8秒粘る
        let since = std::time::Instant::now();

        let mut found = false;
        let mut penalty = std::usize::MAX;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
        let mut cargos = self.cargos.clone();
        cargos.sort_by_key(|x| Reverse(x.volume()));
        let mut sol = Solution::new();
        while since.elapsed().as_secs_f32() < 1.8 {
            let seed = rng.gen_range(0, 2);
            match self.solve_for_cargo(&mut cargos, seed) {
                Ok(solution) => {
                    found = true;
                    if solution.penalty(self.d) < penalty {
                        sol = solution;
                    }
                }
                Err(solution) => {
                    if !found {
                        sol = solution;
                    }
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
    for (p, (h, w, d, a, f, g)) in enumerate(cgs) {
        for _ in 0..a {
            cargos.push(Cargo::new(p, w, h, d, f != "Y", g != "Y"));
        }
    }

    let mut solver = Solver { w, h, b, d, cargos };
    let solution = solver.solve();
    solution.print_result();
}

// cargo run < tools/in/0000.txt > tools/out/0000.txt
