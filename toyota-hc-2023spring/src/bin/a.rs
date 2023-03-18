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
        x0_max < x1_min && y0_max < y1_min
    }

    fn slide(&self, x: usize, y: usize) -> Self {
        Self {
            x0: self.x0 + x,
            y0: self.y0 + y,
            x1: self.y1 + x,
            y1: self.y1 + y,
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
            flippable: true,
            fragile: false,
        }
    }

    fn volume(&self) -> usize {
        self.w * self.h * self.d
    }

    fn rotate(&self) -> Option<Self> {
        if (!self.flippable && self.orientation > 1) || self.orientation == 5 {
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

#[derive(Debug)]
struct Placement {
    pos: Position,
    item: Item,
}

impl Placement {
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
        self.project_x().intersect(&other.project_x())
            || self.project_y().intersect(&other.project_y())
            || self.project_z().intersect(&other.project_z())
    }

    fn vertices(&self) -> Vec<Position> {
        let proj_z = self.project_z();
        let mut vs = Vec::new();
        vs.push(self.pos.slide(self.item.dim_x(), 0, 0));
        vs.push(self.pos.slide(0, self.item.dim_y(), 0));
        vs
    }

    fn print(&self) {
        if self.item.id == BLOCK {
            return;
        }
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
    packed: Vec<Placement>,
    vertices: Vec<Position>,
}

impl Packer {
    fn new(w: usize, h: usize, d: usize, b: usize) -> Self {
        let mut packer = Self {
            w,
            h,
            d,
            packed: Vec::new(),
            vertices: Vec::new(),
        };
        packer.put_item(packer.block(0, 0, b));
        packer.put_item(packer.block(w - b, 0, b));
        packer.put_item(packer.block(0, h - b, b));
        packer.put_item(packer.block(w - b, h - b, b));
        packer
    }

    fn block(&self, x: usize, y: usize, b: usize) -> Placement {
        Placement {
            pos: Position { x, y, z: 0 },
            item: Item::new_block(b, b, self.d),
        }
    }

    fn put_item(&mut self, placement: Placement) {
        placement.print();

        let mut vs = placement.vertices();
        self.packed.push(placement);
        for v in vs {
            if v.x == self.w || v.y == self.h {
                continue;
            }
            self.vertices.push(v);
        }
    }

    fn pack(&self, items: Vec<Item>) {
        let mut items = items;
        items.sort();
        eprintln!("{:?}", items);
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

    let packer = Packer::new(w, h, d, b);
    packer.pack(items);
    // eprintln!("{:?}", packer);
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
                x1: 9,
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
}
