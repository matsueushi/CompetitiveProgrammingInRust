use proconio::{input, source::line::LineSource};
use std::{
    io::{BufReader, Stdin},
    process,
};

#[derive(Clone, Copy)]
struct Pos {
    y: usize,
    x: usize,
}

enum Response {
    NotBroken,
    Broken,
    Finish,
    InValid,
}

struct Field {
    n: usize,
    c: usize,
    is_broken: Vec<Vec<bool>>,
    total_cost: usize,
}

impl Field {
    pub fn new(n: usize, c: usize) -> Self {
        Self {
            n,
            c,
            is_broken: vec![vec![false; n]; n],
            total_cost: 0,
        }
    }

    pub fn query(
        &mut self,
        y: usize,
        x: usize,
        power: usize,
        mut source: &mut LineSource<BufReader<Stdin>>,
    ) -> Response {
        self.total_cost += power + self.c;
        println!("{} {} {}", y, x, power);

        input! {
            from &mut source,
            r: i64,
        }

        println!("# responce:{}", r);

        match r {
            0 => Response::NotBroken,
            1 => {
                self.is_broken[y][x] = true;
                Response::Broken
            }
            2 => {
                self.is_broken[y][x] = true;
                Response::Finish
            }
            _ => Response::InValid,
        }
    }
}

struct Solver {
    n: usize,
    c: usize,
    source_pos: Vec<Pos>,
    house_pos: Vec<Pos>,
    field: Field,
    source: LineSource<BufReader<Stdin>>,
}

impl Solver {
    pub fn new(n: usize, source_pos: Vec<Pos>, house_pos: Vec<Pos>, c: usize) -> Self {
        let stdin = std::io::stdin();
        let source = LineSource::new(BufReader::new(stdin));

        Self {
            n,
            c,
            source_pos,
            house_pos,
            field: Field::new(n, c),
            source,
        }
    }

    pub fn solve(&mut self) {
        for house in self.house_pos.clone() {
            self.mov(house, self.source_pos[0]);
        }
    }

    pub fn mov(&mut self, start: Pos, goal: Pos) {
        println!(
            "# move from ({}, {}) to {} {}",
            start.y, start.x, goal.y, goal.x
        );

        if start.y < goal.y {
            for y in start.y..=goal.y {
                self.destruct(y, start.x);
            }
        } else {
            for y in (goal.y..=start.y).rev() {
                self.destruct(y, start.x)
            }
        }

        if start.x < goal.x {
            for x in start.x..=goal.x {
                self.destruct(goal.y, x);
            }
        } else {
            for x in (goal.x..=start.x).rev() {
                self.destruct(goal.y, x);
            }
        }
    }

    pub fn destruct(&mut self, y: usize, x: usize) {
        const POWER: usize = 100;
        while !self.field.is_broken[y][x] {
            let result = self.field.query(y, x, POWER, &mut self.source);
            match result {
                Response::Finish => {
                    eprintln!("total_cost={}", self.field.total_cost);
                    process::exit(0);
                }
                Response::InValid => {
                    eprintln!("invalid: y={}, x={}", y, x);
                    process::exit(1);
                }
                _ => {}
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        w: usize,
        k: usize,
        c: usize,
        ab: [(usize, usize); w],
        cd: [(usize, usize); k],
    }

    let mut source_pos = Vec::new();
    let mut house_pos = Vec::new();
    for (a, b) in ab {
        source_pos.push(Pos { y: a, x: b });
    }
    for (c, d) in cd {
        house_pos.push(Pos { y: c, x: d });
    }

    let mut solver = Solver::new(n, source_pos, house_pos, c);
    solver.solve();
}

// ../target/release/tester cargo run < tools/in/0000.txt > tools/out/0000.txt
