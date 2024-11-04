use macroquad::math::*;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Hex {
    pub fn from_cube(q: i32, r: i32, s: i32) -> Self {
        assert_eq!(q + r + s, 0);
        Hex { q, r, s }
    }

    pub const fn from_axial(q: i32, r: i32) -> Self {
        Hex { q, r, s: -q - r }
    }

    pub fn neighbor(&self, dir: Dir) -> Hex {
        *self + Dir::CUBE_DIR[dir.to_usize()]
    }

    pub fn neighbor_from_index(&self, index: usize) -> Hex {
        *self + Dir::CUBE_DIR[index]
    }

    pub fn ring(&self, radius: i32) -> Vec<Hex> {
        let mut results = Vec::new();
        let mut h = *self + Dir::CUBE_DIR[4] * radius;

        for i in 0..6 {
            for _ in 0..radius {
                results.push(h);
                h += Dir::CUBE_DIR[i];
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

impl Hex {
    pub fn length(&self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }

    pub fn distance(self, _rhs: Hex) -> i32 {
        (self - _rhs).length()
    }
}

pub struct Orientation {
    pub f: Mat2,
    pub start_angle: f64,
}

impl Orientation {
    pub const LAYOUT_POINTY: Orientation = Orientation {
        f: mat2(vec2(SQRT_3, 0.0), vec2(SQRT_3 / 2.0, 3.0 / 2.0)),
        start_angle: 0.5 * std::f64::consts::PI,
    };
}

pub struct Layout {
    pub orientation: Orientation,
    pub size: Vec2,
    pub origin: Vec2,
}

impl Layout {
    pub fn hex_to_pixel(&self, h: Hex) -> Vec2 {
        let mat = &self.orientation;
        let size = self.size;
        let origin = self.origin;

        mat.f * vec2(h.q as f32, h.r as f32) * size + origin
    }
}

pub enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Dir {
    pub const CUBE_DIR: [Hex; 6] = [
        Hex { q: 1, r: 0, s: -1 },
        Hex { q: 1, r: -1, s: 0 },
        Hex { q: 0, r: -1, s: 1 },
        Hex { q: -1, r: 0, s: 1 },
        Hex { q: -1, r: 1, s: 0 },
        Hex { q: 0, r: 1, s: -1 },
    ];

    pub const AXIAL_DIR: [Hex; 6] = [
        Hex::from_axial(1, 0),
        Hex::from_axial(1, -1),
        Hex::from_axial(0, -1),
        Hex::from_axial(-1, 0),
        Hex::from_axial(-1, 1),
        Hex::from_axial(0, 1),
    ];

    pub fn from_usize(n: usize) -> Self {
        assert!(0 < n && n < 6);

        match n {
            0 => Dir::East,
            1 => Dir::SouthEast,
            2 => Dir::SouthWest,
            3 => Dir::West,
            4 => Dir::NorthWest,
            5 => Dir::NorthEast,
            _ => unreachable!(),
        }
    }

    pub fn to_usize(self) -> usize {
        match self {
            Dir::East => 0,
            Dir::SouthEast => 1,
            Dir::SouthWest => 2,
            Dir::West => 3,
            Dir::NorthWest => 4,
            Dir::NorthEast => 5,
        }
    }
}
