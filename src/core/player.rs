use std::collections::HashMap;

use crate::core::hex::*;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum PlayerState {
    Standing(Hex),
    Flat(Hex, Hex),
    Dead,
}

impl PlayerState {
    pub fn next_state(&self, direction: HexDirection) -> Self {
        let delta = direction.to_hex();

        match self {
            PlayerState::Standing(head) => PlayerState::Flat(*head + delta * 2, *head + delta),
            PlayerState::Flat(head, tail) => {
                let diff = HexDirection::get_dir_from_to(*tail, *head);

                if diff == direction {
                    PlayerState::Standing(*head + delta)
                } else if diff == direction.opposite() {
                    PlayerState::Standing(*tail + delta)
                } else {
                    PlayerState::Flat(*head + delta, *tail + delta)
                }
            }
            _ => PlayerState::Dead,
        }
    }

    pub fn next_state_in_map(&self, direction: HexDirection, hexmap: &HashMap<Hex, bool>) -> Self {
        let delta = direction.to_hex();

        let state = self.next_state(direction);

        match state {
            PlayerState::Standing(hex) => {
                if hexmap.contains_key(&hex) {
                    PlayerState::Standing(hex)
                } else {
                    PlayerState::Dead
                }
            }
            PlayerState::Flat(head, tail) => {
                if hexmap.contains_key(&head) && hexmap.contains_key(&tail) {
                    PlayerState::Flat(head, tail)
                } else {
                    PlayerState::Dead
                }
            }
            _ => PlayerState::Dead,
        }
    }
}
