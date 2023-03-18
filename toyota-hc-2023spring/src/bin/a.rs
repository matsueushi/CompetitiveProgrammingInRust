#![allow(unused)]

const BLOCK: usize = 10000;

use itertools::enumerate;
use proconio::input;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Vector3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Vector3D {
    fn slide(&self, x: usize, y: usize, z: usize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

type Space = Vector3D;
type Position = Vector3D;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Rect {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Rect {
    fn intersect(&self, other: &Self) -> bool {
        let x0_max = self.x0.max(other.x0);
        let x1_min = self.x1.min(other.x1);
        let y0_max = self.y0.max(other.y0);
        let y1_min = self.y1.min(other.y1);
        // eprintln!("{} {} {} {}", x0_max, x1_min, y0_max, y1_min);
        x0_max < x1_min && y0_max < y1_min
    }

    fn intersect_area(&self, other: &Self) -> usize {
        let x0_max = self.x0.max(other.x0);
        let x1_min = self.x1.min(other.x1);
        let y0_max = self.y0.max(other.y0);
        let y1_min = self.y1.min(other.y1);
        if x0_max < x1_min && y0_max < y1_min {
            (x1_min - x0_max) * (y1_min - y0_max)
        } else {
            0
        }
    }

    fn slide(&self, x: usize, y: usize) -> Self {
        Self {
            x0: x + self.x0,
            y0: y + self.y0,
            x1: x + self.x1,
            y1: y + self.y1,
        }
    }
}

/// 荷物
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Item {
    id: usize,
    w: usize,
    h: usize,
    d: usize,
    orientation: usize,
    flippable: bool,
    fragile: bool,
}

impl Item {
    fn new_block(w: usize, h: usize, d: usize) -> Self {
        Self {
            id: BLOCK,
            w,
            h,
            d,
            orientation: 0,
            flippable: false,
            fragile: true,
        }
    }

    fn volume(&self) -> usize {
        self.w * self.h * self.d
    }

    fn rotate(&self) -> Option<Self> {
        if (!self.flippable && self.orientation >= 1) || self.orientation == 5 {
            None
        } else {
            Some(Self {
                id: self.id,
                w: self.w,
                h: self.h,
                d: self.d,
                orientation: self.orientation + 1,
                flippable: self.flippable,
                fragile: self.fragile,
            })
        }
    }

    fn dim_x(&self) -> usize {
        match self.orientation {
            0 => self.w,
            1 => self.h,
            2 => self.d,
            3 => self.h,
            4 => self.d,
            _ => self.w,
        }
    }

    fn dim_y(&self) -> usize {
        match self.orientation {
            0 => self.h,
            1 => self.w,
            2 => self.h,
            3 => self.d,
            4 => self.w,
            _ => self.d,
        }
    }

    fn dim_z(&self) -> usize {
        match self.orientation {
            0 => self.d,
            1 => self.d,
            2 => self.w,
            3 => self.w,
            4 => self.h,
            _ => self.h,
        }
    }

    fn project_x(&self, pos: &Position) -> Rect {
        Rect {
            x0: 0,
            y0: 0,
            x1: self.dim_y(),
            y1: self.dim_z(),
        }
        .slide(pos.y, pos.z)
    }

    fn project_y(&self, pos: &Position) -> Rect {
        Rect {
            x0: 0,
            y0: 0,
            x1: self.dim_x(),
            y1: self.dim_z(),
        }
        .slide(pos.x, pos.z)
    }

    fn project_z(&self, pos: &Position) -> Rect {
        Rect {
            x0: 0,
            y0: 0,
            x1: self.dim_x(),
            y1: self.dim_y(),
        }
        .slide(pos.x, pos.y)
    }

    fn space(&self) -> Space {
        Space {
            x: self.dim_x(),
            y: self.dim_y(),
            z: self.dim_z(),
        }
    }

    fn ground_area(&self) -> usize {
        self.dim_x() * self.dim_y()
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (!self.fragile, self.volume())
            .cmp(&(!other.fragile, other.volume()))
            .reverse()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
struct Placement {
    pos: Position,
    item: Item,
}

impl Placement {
    fn is_block(&self) -> bool {
        self.item.id == BLOCK
    }

    fn x_lower(&self) -> usize {
        self.pos.x
    }

    fn x_upper(&self) -> usize {
        self.pos.x + self.item.dim_x()
    }

    fn y_lower(&self) -> usize {
        self.pos.y
    }

    fn y_upper(&self) -> usize {
        self.pos.y + self.item.dim_y()
    }

    fn z_lower(&self) -> usize {
        self.pos.z
    }

    fn z_upper(&self) -> usize {
        self.pos.z + self.item.dim_z()
    }

    fn project_x(&self) -> Rect {
        self.item.project_x(&self.pos)
    }

    fn project_y(&self) -> Rect {
        self.item.project_y(&self.pos)
    }

    fn project_z(&self) -> Rect {
        self.item.project_z(&self.pos)
    }

    /// 長方形の交差判定をする
    fn intersect(&self, other: &Self) -> bool {
        // eprintln!(
        //     "{:?} {:?} {:?} {:?} {:?} {:?}",
        //     self.project_x(),
        //     other.project_x(),
        //     self.project_y(),
        //     other.project_y(),
        //     self.project_z(),
        //     other.project_z()
        // );
        self.project_x().intersect(&other.project_x())
            && self.project_y().intersect(&other.project_y())
            && self.project_z().intersect(&other.project_z())
    }

    /// 上に乗っているか、載せられるか
    /// 地面に接しているときは地面の面積
    fn ground_contact_area(&self, under: &Self) -> Result<usize, ()> {
        // underと接しているか
        if self.z_lower() != under.z_upper() {
            return Ok(0);
        }
        let ground = self.project_z();
        let top = under.project_z();
        if ground.intersect(&top) {
            if under.item.fragile {
                Err(())
            } else {
                Ok(ground.intersect_area(&top))
            }
        } else {
            Ok(0)
        }
    }

    fn vertices(&self) -> Vec<Position> {
        let proj_z = self.project_z();
        let mut vs = Vec::new();
        vs.push(self.pos.slide(self.item.dim_x(), 0, 0));
        vs.push(self.pos.slide(0, self.item.dim_y(), 0));
        if !self.is_block() {
            vs.push(self.pos.slide(0, 0, self.item.dim_z()));
        }
        vs
    }

    fn print(&self) {
        println!(
            "{} {} {} {} {}",
            self.item.id, self.item.orientation, self.pos.x, self.pos.y, self.pos.z
        );
    }
}

#[derive(Debug)]
struct Packer {
    w: usize,
    h: usize,
    d: usize,
    blocks: Vec<Placement>,
    packed: Vec<Placement>,
    vertices: Vec<Position>,
}

impl Packer {
    fn new(w: usize, h: usize, d: usize, b: usize) -> Self {
        // 地面を配置する
        let mut blocks = Vec::new();
        blocks.push(Placement {
            pos: Position { x: 0, y: 0, z: 0 },
            item: Item {
                id: BLOCK,
                w,
                h,
                d: 0, // depth は 0
                orientation: 0,
                flippable: true,
                fragile: false,
            },
        });

        let mut packer = Self {
            w,
            h,
            d,
            blocks,
            packed: Vec::new(),
            vertices: Vec::new(),
        };
        packer.add_block(0, 0, b);
        packer.add_block(w - b, 0, b);
        packer.add_block(0, h - b, b);
        packer.add_block(w - b, h - b, b);
        packer
    }

    fn add_block(&mut self, x: usize, y: usize, b: usize) {
        let placement = Placement {
            pos: Position { x, y, z: 0 },
            item: Item::new_block(b, b, self.d),
        };
        self.add_vertices(placement.vertices());
        self.blocks.push(placement);
    }

    fn add_vertices(&mut self, vs: Vec<Position>) {
        for v in vs {
            if v.x == self.w || v.y == self.h {
                continue;
            }
            self.vertices.push(v);
        }
    }

    fn put_item(&mut self, placement: Placement) {
        // eprintln!("put_item {:?}", placement);
        let mut vs = placement.vertices();
        self.packed.push(placement);
        self.add_vertices(vs);
    }

    fn check_allocation(&self, pos: &Position, item: &Item) -> Option<Placement> {
        let placement = Placement {
            pos: *pos,
            item: *item,
        };
        // 境界チェック
        if placement.x_upper() > self.w || placement.y_upper() > self.h {
            return None;
        }

        // 交差するかどうかを調べる
        let mut contact_area = 0; // 接触面積
        for p in self.blocks.iter().chain(self.packed.iter()) {
            if placement.intersect(&p) {
                // eprintln!("intersected {:?}, {:?}", p, placement);
                return None;
            }

            match placement.ground_contact_area(&p) {
                Ok(area) => {
                    // eprintln!("{:?} {:?} {}", p, placement, area);
                    contact_area += area;
                }
                Err(_) => {
                    return None;
                }
            }
        }
        // 面積チェック
        let required = (item.ground_area() as f64 * 0.6).round() as usize;
        if contact_area < required {
            // eprintln!(
            //     "area limitation is not satisfied {} {}",
            //     contact_area, required
            // );
            None
        } else {
            Some(placement)
        }
    }

    fn penalty(&self) -> usize {
        let mut score = 1000;
        // max height
        let mut max_h = 0;
        let mut overflow_vol = 0;
        for p in &self.packed {
            let h = p.z_upper();
            max_h = max_h.max(h);
            if h > self.d {
                overflow_vol += p.item.volume();
            }
        }
        score += max_h;
        for i in 0..self.packed.len() {
            for j in i + 1..self.packed.len() {
                if self.packed[i].item.id > self.packed[j].item.id {
                    score += 1000;
                }
            }
        }
        if max_h > self.d {
            score += 1_000_000 + 1000 * overflow_vol;
        }
        score
    }

    fn pack_item(&mut self, vertex: &Position, item: &Item) -> Option<Placement> {
        let mut item = Some(*item);
        loop {
            match item {
                Some(it) => {
                    let ret = self.check_allocation(&vertex, &it);
                    if ret.is_some() {
                        return ret;
                    }
                    item = it.rotate();
                }
                None => return None,
            }
        }
        None
    }

    /// 順番通りに並べる
    fn order_items(packed: &Vec<Placement>) -> Vec<Placement> {
        let mut n_items = packed.len();
        let mut items = Vec::new();
        let mut used = vec![false; n_items];
        while n_items > 0 {
            for i in 0..packed.len() {
                let mut top = true;
                if used[i] {
                    continue;
                }
                for j in 0..packed.len() {
                    if i == j || used[j] {
                        continue;
                    }
                    if packed[j].z_lower() >= packed[i].z_upper()
                        && packed[i].project_z().intersect(&packed[j].project_z())
                    {
                        top = false;
                    }
                }

                if top {
                    used[i] = true;
                    items.push(i);
                    n_items -= 1;
                }
            }
        }

        let mut ordered_items = Vec::new();
        for i in items.iter().rev() {
            ordered_items.push(packed[*i]);
        }
        ordered_items
    }

    fn pack(&mut self, items: Vec<Item>) -> Vec<Placement> {
        let mut items = items;
        items.sort();

        let mut success = true;
        for item in &items {
            let vs = self.vertices.clone();
            // もっとも低くなるように積む
            let mut h = std::usize::MAX;
            let mut p = None;
            for v in vs {
                if let Some(placement) = self.pack_item(&v, &item) {
                    let nh = placement.z_upper();
                    if nh < h {
                        h = nh;
                        p = Some(placement);
                    }
                }
            }
            if let Some(placement) = p {
                self.put_item(placement);
            } else {
                success = false;
                break;
            }
        }

        if success {
            eprintln!("success. score = {}", self.penalty());
        }

        Self::order_items(&self.packed)
    }
}

fn main() {
    input! {
        m: usize,
        w: usize,
        h: usize,
        b: usize,
        d: usize,
        cs: [(usize, usize, usize, usize, String, String); m]
    }

    let mut items = Vec::new();
    for (id, (hi, wi, di, ai, fi, gi)) in enumerate(cs) {
        for _ in 0..ai {
            items.push(Item {
                id,
                w: wi,
                h: hi,
                d: di,
                orientation: 0,
                flippable: fi == "Y",
                fragile: gi == "N",
            })
        }
    }

    let mut packer = Packer::new(w, h, d, b);
    let packed = packer.pack(items);
    for p in packed {
        p.print();
    }
}

///
/// 以下はテスト
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_intersect() {
        let rect = Rect {
            x0: 0,
            y0: 0,
            x1: 1,
            y1: 1,
        };
        let rect2 = Rect {
            x0: 1,
            y0: 1,
            x1: 3,
            y1: 3,
        };
        let rect3 = Rect {
            x0: 0,
            y0: 2,
            x1: 3,
            y1: 3,
        };
        assert!(!rect.intersect(&rect2));
        assert!(rect2.intersect(&rect3));
    }

    #[test]
    fn test_rect_slide() {
        let rect = Rect {
            x0: 1,
            y0: 2,
            x1: 3,
            y1: 4,
        };
        let rect = rect.slide(5, 6);
        assert_eq!(
            rect,
            Rect {
                x0: 6,
                y0: 8,
                x1: 8,
                y1: 10,
            }
        );
    }

    #[test]
    fn intersect_boxes_0() {
        let p1 = Placement {
            pos: Position { x: 0, y: 0, z: 0 },
            item: Item {
                id: 0,
                w: 1,
                h: 1,
                d: 1,
                orientation: 0,
                fragile: false,
                flippable: true,
            },
        };
        let p2 = Placement {
            pos: Position { x: 1, y: 1, z: 1 },
            item: Item {
                id: 0,
                w: 1,
                h: 1,
                d: 1,
                orientation: 0,
                fragile: false,
                flippable: true,
            },
        };
        assert!(!p1.intersect(&p2));
    }

    #[test]
    fn intersect_boxes_1() {
        let p1 = Placement {
            pos: Position { x: 0, y: 0, z: 0 },
            item: Item {
                id: 0,
                w: 2,
                h: 2,
                d: 2,
                orientation: 0,
                fragile: false,
                flippable: true,
            },
        };
        let p2 = Placement {
            pos: Position { x: 1, y: 1, z: 1 },
            item: Item {
                id: 0,
                w: 1,
                h: 1,
                d: 1,
                orientation: 0,
                fragile: false,
                flippable: true,
            },
        };
        assert!(p1.intersect(&p2));
    }

    #[test]
    fn test_project() {
        let item = Item {
            id: 0,
            w: 1,
            h: 2,
            d: 3,
            orientation: 0,
            fragile: false,
            flippable: true,
        };
        let pos = Position { x: 1, y: 1, z: 1 };
        assert_eq!(
            item.project_x(&pos),
            Rect {
                x0: 1,
                y0: 1,
                x1: 3,
                y1: 4,
            }
        );
    }

    #[test]
    fn test_ground_contact_area() {
        let p1 = Placement {
            pos: Position { x: 0, y: 0, z: 0 },
            item: Item {
                id: 0,
                w: 2,
                h: 2,
                d: 2,
                orientation: 0,
                fragile: false,
                flippable: true,
            },
        };
        let p2 = Placement {
            pos: Position { x: 1, y: 1, z: 2 },
            item: Item {
                id: 0,
                w: 2,
                h: 2,
                d: 2,
                orientation: 0,
                fragile: false,
                flippable: true,
            },
        };
        assert_eq!(p2.ground_contact_area(&p1), Ok(1));

        let p3 = Placement {
            pos: Position { x: 0, y: 0, z: 0 },
            item: Item {
                id: 0,
                w: 2,
                h: 2,
                d: 2,
                orientation: 0,
                fragile: true,
                flippable: true,
            },
        };

        assert_eq!(p2.ground_contact_area(&p3), Err(()));
    }
}
