use std::collections::HashMap;

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
    pub fn solve_path(&self) -> Option<Vec<PlayerState>> {
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

    pub fn gen() -> Self {
        let mut hexmap = HashMap::new();
        let start = Hex::from_axial(1, 1);
        let goal = Hex::from_axial(6, 7);

        for q in 0..=8 {
            for r in 0..=8 {
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
