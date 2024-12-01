use std::collections::HashMap;

use macroquad::prelude::rand;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use crate::core::game::*;
use crate::core::hex::*;

use super::hex;

pub struct HexMap {
    pub hexmap: HashMap<Hex, bool>,
    pub start: Hex,
    pub goal: Hex,
}

impl HexMap {
    pub fn solve_path(&self, hex_start: Hex) -> Option<Vec<PlayerState>> {
        let mut g = Graph::new();

        let mut start = NodeIndex::default();
        let mut goal = NodeIndex::default();

        let mut idx = HashMap::new();

        for head in self.hexmap.keys() {
            // Standing
            for dir in HEX_DIRECTIONS {
                let cur = PlayerState::Standing(*head);

                let icur = if !idx.contains_key(&cur) {
                    let i = g.add_node(cur);
                    idx.insert(cur, i);
                    i
                } else {
                    idx[&cur]
                };

                if *head == hex_start {
                    start = icur;
                }

                if *head == self.goal {
                    goal = icur;
                }

                let next = cur.next_state_in_map(dir, &self.hexmap);
                if next == PlayerState::Dead {
                    continue;
                }

                let inext = if !idx.contains_key(&next) {
                    let i = g.add_node(next);
                    idx.insert(next, i);
                    i
                } else {
                    idx[&next]
                };

                g.update_edge(icur, inext, 1);
            }

            // Flat
            for dir in HEX_DIRECTIONS {
                let tail = head.neighbor(dir);

                if !self.hexmap.contains_key(&tail) {
                    continue;
                }

                let cur = PlayerState::Flat(*head, tail);

                let icur = if !idx.contains_key(&cur) {
                    let i = g.add_node(cur);
                    idx.insert(cur, i);
                    i
                } else {
                    idx[&cur]
                };

                for dir2 in HEX_DIRECTIONS {
                    let next = cur.next_state_in_map(dir2, &self.hexmap);
                    if next == PlayerState::Dead {
                        continue;
                    }

                    let inext = if !idx.contains_key(&next) {
                        let i = g.add_node(next);
                        idx.insert(next, i);
                        i
                    } else {
                        idx[&next]
                    };

                    g.update_edge(icur, inext, 1);
                }
            }
        }

        let path = astar(&g, start, |finish| finish == goal, |e| *e.weight(), |_| 0);

        match path {
            Some((_, d)) => {
                let mut d = d.clone();
                let mut v = vec![];

                while let Some(i) = d.pop() {
                    let state = g[i];
                    v.push(state);
                }

                Some(v)
            }
            None => None,
        }
    }

    fn is_in_map(&self, hex: Hex) -> bool {
        self.hexmap.contains_key(&hex)
    }

    pub fn dump_map(&self, radius: i32) {
        let s = radius;
        for r in -s..=s {
            for _ in 0..r {
                print!(" ");
            }
            for q in -s..=s {
                let pos = Hex::from_axial(q, r);
                if self.is_in_map(pos) {
                    if pos == self.goal {
                        print!("X ");
                    } else if pos == self.start {
                        print!("A ");
                    } else {
                        print!("* ");
                    }
                } else {
                    print!("- ");
                }
            }
            println!();
        }
        println!();
    }

    pub fn gen() -> Self {
        let mut hexmap = HashMap::new();
        let mut start = Hex::from_axial(0, 0);
        let mut goal = Hex::from_axial(1, 1);

        let mut last_hex = start;

        let mut cnt = 20;
        while cnt > 0 {
            cnt -= 1;
            let mut vdir = Vec::new();
            vdir.push(DIR[HexDirection::to_usize(HexDirection::SW)]);
            vdir.push(DIR[HexDirection::to_usize(HexDirection::SE)]);
            vdir.push(DIR[HexDirection::to_usize(HexDirection::E)]);
            vdir.push(DIR[HexDirection::to_usize(HexDirection::W)]);

            if rand::gen_range(0, 3) == 0 {
                for _ in 0..3 {
                    let p_hex = last_hex + vdir[rand::gen_range(0, vdir.len())];
                    hexmap.insert(p_hex, true);
                }
            }

            let next_hex = last_hex + vdir[rand::gen_range(0, vdir.len())];
            last_hex = next_hex;
            hexmap.insert(last_hex, true);
        }
        goal = last_hex;

        hexmap.insert(start, true);
        hexmap.insert(goal, true);
        for i in 0..6 {
            hexmap.insert(start + DIR[i], true);
            hexmap.insert(goal + DIR[i], true);
        }

        Self {
            hexmap,
            start,
            goal,
        }
    }
}

const PREMADE_MAP_3: [[char; 9]; 9] = [
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', 'A', '*', '.', '.', '.', '.', '.', '.'],
    ['.', '*', '*', '.', '.', '.', '.', '.', '.'],
    ['.', '*', '*', '*', '*', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '*', '.', '*', '*', '.'],
    ['.', '.', '.', '.', '*', '.', '*', '*', '.'],
    ['.', '.', '.', '.', '*', '*', 'X', '*', '.'],
    ['.', '.', '.', '.', '.', '*', '*', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
];

const PREMADE_MAP_2: [[char; 9]; 9] = [
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', 'A', '*', '.', '.', '.', '.', '.', '.'],
    ['.', '*', '*', '.', '.', '.', '.', '.', '.'],
    ['.', '*', '*', '*', '*', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '*', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '*', '*', '*', '*', '.'],
    ['.', '.', '.', '.', '*', '*', 'X', '*', '.'],
    ['.', '.', '.', '.', '.', '*', '*', '.', '.'],
];

const PREMADE_MAP_1: [[char; 9]; 9] = [
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '*', '*', '*', '*', '.'],
    ['.', '.', 'A', '*', '.', '.', '*', '*', '.'],
    ['.', '.', '*', '*', '.', '.', '*', '*', '.'],
    ['.', '.', '*', '*', '.', '*', 'X', '*', '.'],
    ['.', '*', '*', '*', '.', '*', '*', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
];

fn load_map(arr: [[char; 9]; 9]) -> HexMap {
    let mut hexmap = HashMap::new();
    let mut start = Hex::from_axial(0, 0);
    let mut goal = Hex::from_axial(0, 0);

    for r in 0..9 {
        for q in 0..9 {
            let hex = Hex::from_axial(q as i32, r as i32);
            match arr[r][q] {
                '*' => {
                    hexmap.insert(hex, true);
                }
                'A' => {
                    hexmap.insert(hex, true);
                    start = hex;
                }
                'X' => {
                    hexmap.insert(hex, true);
                    goal = hex;
                }
                _ => {}
            }
        }
    }

    HexMap {
        hexmap,
        start,
        goal,
    }
}
