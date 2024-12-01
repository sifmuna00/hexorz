use std::collections::HashMap;

use crate::core::hex::*;
use crate::core::map::*;
use crate::HEXES_SIZE;
use macroquad::prelude::*;

use super::hex;

const MAP_ZOOM: f32 = 2.0;

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

pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
    GameWon,
}

pub struct Game {
    pub layout: Layout,
    pub player_state: PlayerState,
    pub map: HexMap,
    tile_texture: Texture2D,
    standing_texture: Texture2D,
    flat_diag_main_texture: Texture2D,
    flat_diag_other_texture: Texture2D,
    flat_w_texture: Texture2D,
    flat_e_texture: Texture2D,
}

impl Game {
    pub async fn init() -> Self {
        set_pc_assets_folder("assets");
        let tile_texture: Texture2D = load_texture("hex_tile.png").await.unwrap();
        let standing_texture: Texture2D = load_texture("hex_standing.png").await.unwrap();
        let flat_diag_main_texture: Texture2D =
            load_texture("hex_flat_diag_main.png").await.unwrap();
        let flat_diag_other_texture: Texture2D =
            load_texture("hex_flat_diag_other.png").await.unwrap();
        let flat_w_texture: Texture2D = load_texture("hex_flat_w.png").await.unwrap();
        let flat_e_texture: Texture2D = load_texture("hex_flat_e.png").await.unwrap();

        let pointy: Layout = Layout {
            orientation: Orientation::LAYOUT_POINTY,
            // Point(W/sqrt(3), H/2)
            size: Vec2 {
                x: 31.0 / SQRT_3,
                y: 21.0 / 2.0,
            },
            origin: Vec2 {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
            },
        };

        let game_map = HexMap::gen();
        game_map.dump_map(5);

        Game {
            layout: pointy.clone(),
            player_state: PlayerState::Standing(game_map.start),
            map: game_map,
            tile_texture,
            standing_texture,
            flat_diag_main_texture,
            flat_diag_other_texture,
            flat_w_texture,
            flat_e_texture,
        }
    }

    pub fn update_map(&mut self) {
        let game_map = HexMap::gen();

        self.player_state = PlayerState::Standing(game_map.start);
        self.map = game_map;
    }

    fn draw_flat(&self, state: PlayerState) {
        let head = match state {
            PlayerState::Flat(head, _) => head,
            _ => Hex::from_axial(0, 0),
        };
        let tail = match state {
            PlayerState::Flat(_, tail) => tail,
            _ => Hex::from_axial(0, 0),
        };

        let head_pixel = self.layout.hex_to_pixel(head) + vec2(16.0, 11.0);
        let tail_pixel = self.layout.hex_to_pixel(tail) + vec2(16.0, 11.0);

        draw_line(
            tail_pixel.x,
            tail_pixel.y,
            head_pixel.x,
            head_pixel.y,
            HEXES_SIZE / 10.0,
            GREEN,
        );

        draw_circle(head_pixel.x, head_pixel.y, HEXES_SIZE / 5.0, RED);
    }

    fn draw_flat_to_stading(&self, flat: PlayerState, standing: PlayerState) {
        let head = match flat {
            PlayerState::Flat(head, _) => head,
            _ => Hex::from_axial(0, 0),
        };

        let next = match standing {
            PlayerState::Standing(next) => next,
            _ => Hex::from_axial(0, 0),
        };

        let head_pixel = self.layout.hex_to_pixel(head) + vec2(16.0, 11.0);
        let next_pixel = self.layout.hex_to_pixel(next) + vec2(16.0, 11.0);

        draw_line(
            head_pixel.x,
            head_pixel.y,
            next_pixel.x,
            next_pixel.y,
            HEXES_SIZE / 10.0,
            GREEN,
        );

        draw_circle(next_pixel.x, next_pixel.y, HEXES_SIZE / 5.0, RED);
    }

    pub fn draw_ans(&self, hex_start: Hex) {
        let path = self.map.solve_path(hex_start);

        if let Some(path) = path {
            let mut path = path;
            path.reverse();

            for i in 1..path.len() {
                match path[i] {
                    PlayerState::Standing(_) => {
                        self.draw_flat_to_stading(path[i - 1], path[i]);
                    }
                    _ => {
                        self.draw_flat(path[i]);
                    }
                }
            }
        } else {
            println!("No path found");
        }
    }

    fn move_player(&mut self, direction: HexDirection) {
        let delta = direction.to_hex();
        if delta == Hex::from_axial(0, 0) {
            return;
        }

        let hexmap = &self.map.hexmap;

        self.player_state = match self.player_state {
            PlayerState::Standing(head) => PlayerState::Flat(head + delta * 2, head + delta),
            PlayerState::Flat(head, tail) => {
                let diff = HexDirection::get_dir_from_to(tail, head);

                if diff == direction {
                    PlayerState::Standing(head + delta)
                } else if diff == direction.opposite() {
                    PlayerState::Standing(tail + delta)
                } else {
                    PlayerState::Flat(head + delta, tail + delta)
                }
            }
            _ => PlayerState::Dead,
        };

        self.player_state = match self.player_state {
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
        };
    }

    pub fn update(&mut self) {
        if let Some(key) = get_last_key_pressed() {
            let dir: Option<HexDirection> = match key {
                KeyCode::W => Some(HexDirection::NW),
                KeyCode::E => Some(HexDirection::NE),
                KeyCode::D => Some(HexDirection::E),
                KeyCode::A => Some(HexDirection::W),
                KeyCode::Z => Some(HexDirection::SW),
                KeyCode::X => Some(HexDirection::SE),
                _ => None,
            };

            if let Some(dir) = dir {
                self.move_player(dir);
            }
        }
    }

    pub fn draw_tiles(&self, texture: &Texture2D) {
        let hexmap = &self.map.hexmap;
        let goal = self.map.goal;
        let v = hexmap.keys().collect::<Vec<_>>();
        let mut hexes = v.iter().map(|hex| hex.to_offset()).collect::<Vec<_>>();

        hexes.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        for hex in hexes {
            let h = Hex::from_offset(hex);
            let pixel = self.layout.hex_to_pixel(h);

            if h == goal {
                // draw_texture(texture, pixel.x, pixel.y, BLACK);
                continue;
            } else {
                draw_texture(texture, pixel.x, pixel.y, WHITE);
            }
        }
    }

    // 9b4747
    pub fn draw_player_hex(
        &self,
        standing_texture: &Texture2D,
        flat_diag_main_texture: &Texture2D,
        flat_diag_other_texture: &Texture2D,
        flat_e_texture: &Texture2D,
        flat_w_texture: &Texture2D,
    ) {
        match self.player_state {
            PlayerState::Standing(hex) => {
                let pixel = self.layout.hex_to_pixel(hex) + vec2(0.0, -32.0);
                draw_texture(standing_texture, pixel.x, pixel.y, WHITE);
            }
            PlayerState::Flat(head, tail) => {
                let dir = HexDirection::get_dir_from_to(head, tail);

                match dir {
                    HexDirection::NW => {
                        let pixel = self.layout.hex_to_pixel(tail) + vec2(-4.0, -13.0);
                        draw_texture(flat_diag_main_texture, pixel.x, pixel.y, WHITE);
                    }
                    HexDirection::SE => {
                        let pixel = self.layout.hex_to_pixel(head) + vec2(-4.0, -13.0);
                        draw_texture(flat_diag_main_texture, pixel.x, pixel.y, WHITE);
                    }
                    HexDirection::NE => {
                        let pixel = self.layout.hex_to_pixel(head) + vec2(-4.0, -28.0);
                        draw_texture(flat_diag_other_texture, pixel.x, pixel.y, WHITE);
                    }
                    HexDirection::SW => {
                        let pixel = self.layout.hex_to_pixel(tail) + vec2(-4.0, -28.0);
                        draw_texture(flat_diag_other_texture, pixel.x, pixel.y, WHITE);
                    }
                    HexDirection::W => {
                        let pixel = self.layout.hex_to_pixel(tail) + vec2(2.0, -9.0);
                        draw_texture(flat_w_texture, pixel.x, pixel.y, WHITE);
                    }
                    HexDirection::E => {
                        let pixel = self.layout.hex_to_pixel(head) + vec2(2.0, -9.0);
                        draw_texture(flat_e_texture, pixel.x, pixel.y, WHITE);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn draw(&self, is_debug: bool) {
        set_camera(&Camera2D {
            zoom: vec2(
                MAP_ZOOM / screen_width() * 2.0,
                MAP_ZOOM / screen_height() * 2.0,
            ),
            target: self.get_center(),
            ..Default::default()
        });

        self.draw_tiles(&self.tile_texture);
        self.draw_player_hex(
            &self.standing_texture,
            &self.flat_diag_main_texture,
            &self.flat_diag_other_texture,
            &self.flat_e_texture,
            &self.flat_w_texture,
        );

        if is_debug {
            if let PlayerState::Standing(hex) = self.player_state {
                self.draw_ans(hex);
            }

            // self.draw_ans(self.map.start);
        }
    }

    fn get_center(&self) -> Vec2 {
        let hexmap = &self.map.hexmap;
        let mut top_left = vec2(f32::MAX, f32::MAX);
        let mut bottom_right = vec2(f32::MIN, f32::MIN);

        for hex in hexmap.keys() {
            let pixel = self.layout.hex_to_pixel(*hex);

            top_left.x = top_left.x.min(pixel.x);
            top_left.y = top_left.y.min(pixel.y);

            bottom_right.x = bottom_right.x.max(pixel.x);
            bottom_right.y = bottom_right.y.max(pixel.y);
        }

        Vec2 {
            x: (top_left.x + bottom_right.x) / 2.0,
            y: (top_left.y + bottom_right.y) / 2.0,
        }
    }
}
