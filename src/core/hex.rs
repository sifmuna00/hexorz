use macroquad::math::*;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub const SQRT_3: f32 = 1.732_050_8_f32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

#[allow(dead_code)]
impl Hex {
    pub fn from_cube(q: i32, r: i32, s: i32) -> Self {
        assert_eq!(q + r + s, 0);
        Hex { q, r, s }
    }

    pub const fn from_axial(q: i32, r: i32) -> Self {
        Hex { q, r, s: -q - r }
    }

    pub fn from_offset(offset: (i32, i32)) -> Self {
        let (col, row) = offset;
        let q = col - (row + (row & 1)) / 2;
        let r = row;
        Hex::from_axial(q, r)
    }

    pub const fn to_offset(&self) -> (i32, i32) {
        let col = self.q + (self.r + (self.r & 1)) / 2;
        let row = self.r;
        (col, row)
    }

    pub fn neighbor(&self, dir: HexDirection) -> Hex {
        *self + DIR[dir.to_usize()]
    }

    pub fn neighbor_from_index(&self, index: usize) -> Hex {
        *self + DIR[index]
    }

    pub fn ring(&self, radius: i32) -> Vec<Hex> {
        let mut results = Vec::new();
        let mut h = *self + DIR[4] * radius;

        for i in 0..6 {
            for _ in 0..radius {
                results.push(h);
                h += DIR[i];
            }
        }

        if radius == 0 {
            results.push(*self);
        }

        results
    }

    pub fn spiral(&self, radius: i32) -> Vec<Hex> {
        let mut results = Vec::new();
        results.push(*self);

        for r in 1..=radius {
            results.extend(self.ring(r));
        }

        results
    }
}

impl Add<Hex> for Hex {
    type Output = Hex;

    fn add(self, _rhs: Hex) -> Hex {
        Hex {
            q: self.q + _rhs.q,
            r: self.r + _rhs.r,
            s: self.s + _rhs.s,
        }
    }
}

impl AddAssign<Hex> for Hex {
    fn add_assign(&mut self, _rhs: Hex) {
        *self = *self + _rhs;
    }
}

impl Sub<Hex> for Hex {
    type Output = Hex;

    fn sub(self, _rhs: Hex) -> Hex {
        Hex {
            q: self.q - _rhs.q,
            r: self.r - _rhs.r,
            s: self.s - _rhs.s,
        }
    }
}

impl SubAssign<Hex> for Hex {
    fn sub_assign(&mut self, _rhs: Hex) {
        *self = *self - _rhs;
    }
}

impl Mul<i32> for Hex {
    type Output = Hex;

    fn mul(self, _rhs: i32) -> Hex {
        Hex {
            q: self.q * _rhs,
            r: self.r * _rhs,
            s: self.s * _rhs,
        }
    }
}

impl MulAssign<i32> for Hex {
    fn mul_assign(&mut self, _rhs: i32) {
        *self = *self * _rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Orientation {
    pub f: Mat2,
    pub f_inv: Mat2,
    pub start_angle: f64,
}

impl Orientation {
    pub const LAYOUT_POINTY: Orientation = Orientation {
        f: Mat2::from_cols_array(&[SQRT_3, 0.0, SQRT_3 / 2.0, 3.0 / 2.0]),
        f_inv: Mat2::from_cols_array(&[SQRT_3 / 3.0, 0.0, -1.0 / 3.0, 2.0 / 3.0]),
        start_angle: 0.5 * std::f64::consts::PI,
    };
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub orientation: Orientation,
    pub size: Vec2,
    pub origin: Vec2,
}

impl Layout {
    pub fn hex_to_pixel(&self, hex: Hex) -> Vec2 {
        let mat = &self.orientation;
        let size = self.size;
        let origin = self.origin;

        let res = mat.f * vec2(hex.q as f32, hex.r as f32) * size + origin;
        res
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HexDirection {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

pub const DIR: [Hex; 6] = [
    Hex::from_axial(1, 0),
    Hex::from_axial(1, -1),
    Hex::from_axial(0, -1),
    Hex::from_axial(-1, 0),
    Hex::from_axial(-1, 1),
    Hex::from_axial(0, 1),
];

pub const HEX_DIRECTIONS: [HexDirection; 6] = [
    HexDirection::E,
    HexDirection::SE,
    HexDirection::SW,
    HexDirection::W,
    HexDirection::NW,
    HexDirection::NE,
];

impl HexDirection {
    pub fn from_usize(n: usize) -> Self {
        assert!(n < 6);

        match n {
            0 => HexDirection::E,
            1 => HexDirection::NE,
            2 => HexDirection::NW,
            3 => HexDirection::W,
            4 => HexDirection::SW,
            5 => HexDirection::SE,
            _ => unreachable!(),
        }
    }

    pub fn to_usize(self) -> usize {
        match self {
            HexDirection::E => 0,
            HexDirection::NE => 1,
            HexDirection::NW => 2,
            HexDirection::W => 3,
            HexDirection::SW => 4,
            HexDirection::SE => 5,
        }
    }

    pub fn to_hex(self) -> Hex {
        DIR[self.to_usize()]
    }

    pub fn get_dir_from_to(from: Hex, to: Hex) -> Self {
        let diff = to - from;
        for dir in 0..6 {
            if diff == DIR[dir] {
                return HexDirection::from_usize(dir);
            }
        }
        panic!("impossible positions: {:?}, {:?}", from, to);
    }

    pub fn opposite(&self) -> Self {
        match self {
            HexDirection::E => HexDirection::W,
            HexDirection::SE => HexDirection::NW,
            HexDirection::SW => HexDirection::NE,
            HexDirection::W => HexDirection::E,
            HexDirection::NW => HexDirection::SE,
            HexDirection::NE => HexDirection::SW,
        }
    }
}
