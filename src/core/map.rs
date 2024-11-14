use std::collections::HashSet;
use std::{collections::HashMap, hash::Hash, vec};

use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use crate::core::hex::*;
use crate::core::player::*;

pub struct HexMap {
    pub hexmap: HashMap<Hex, bool>,
    pub start: Hex,
    pub goal: Hex,
}

impl HexMap {
    pub fn solve_path(&self) {
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

                if *head == self.start {
                    start = icur;
                }

                if *head == self.goal {
                    goal = icur;
                }

                let next = cur.next_state(dir);

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

                let cur = PlayerState::Flat(*head, tail);

                let icur = if !idx.contains_key(&cur) {
                    let i = g.add_node(cur);
                    idx.insert(cur, i);
                    i
                } else {
                    idx[&cur]
                };

                for dir2 in HEX_DIRECTIONS {
                    let next = cur.next_state(dir2);

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

        println!("Path: {:?}", path);

        let mut mp = HashSet::new();
        for i in g.node_indices() {
            mp.insert(g[i]);
            eprintln!("{:?}", g[i]);
        }

        println!("Cnt: {}", mp.len());
        println!("NodeCount: {}", g.node_count());
    }

    pub fn gen() -> Self {
        let mut hexmap = HashMap::new();
        let start = Hex::from_axial(1, 1);
        let goal = Hex::from_axial(4, 1);

        for q in 1..=4 {
            for r in 1..=1 {
                let hex = Hex::from_axial(q, r);
                hexmap.insert(hex, true);
            }
        }

        Self {
            hexmap,
            start,
            goal,
        }
    }
}
