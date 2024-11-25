use ::core::f32;
use std::{collections::HashMap, env::consts};

use macroquad::prelude::*;

mod core;

use core::{hex::*, map::HexMap, player::PlayerState};

enum GameState {
    MainMenu,
    Playing,
    GameOver,
    GameWon,
}

struct Game {
    layout: Layout,
    hexmap: HashMap<Hex, bool>,
    player_state: PlayerState,
    start: Hex,
    goal: Hex,
}

const HEXES_SIZE: f32 = 32.0;

impl Game {
    fn draw(&self) {
        for (hex, _) in self.hexmap.iter() {
            let pixel = self.layout.hex_to_pixel(*hex);
            draw_hexagon(pixel.x, pixel.y, HEXES_SIZE, 1.0, true, RED, WHITE);
        }

        self.draw_goal();
        self.draw_player();
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

        let head_pixel = self.layout.hex_to_pixel(head);
        let tail_pixel = self.layout.hex_to_pixel(tail);

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

        let head_pixel = self.layout.hex_to_pixel(head);
        let next_pixel = self.layout.hex_to_pixel(next);

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

    fn draw_ans(&self) {
        let hm = HexMap {
            hexmap: self.hexmap.clone(),
            start: self.start,
            goal: self.goal,
        };

        let path = hm.solve_path();

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
        }
    }

    fn draw_player(&self) {
        match self.player_state {
            PlayerState::Standing(hex) => {
                let pixel = self.layout.hex_to_pixel(hex);
                draw_circle(pixel.x, pixel.y, HEXES_SIZE / 2.0, BLUE);
            }
            PlayerState::Flat(head, tail) => {
                let head_pixel = self.layout.hex_to_pixel(head);
                let tail_pixel = self.layout.hex_to_pixel(tail);
                draw_circle(head_pixel.x, head_pixel.y, HEXES_SIZE / 2.0, BLUE);
                draw_circle(tail_pixel.x, tail_pixel.y, HEXES_SIZE / 2.0, YELLOW);
            }
            _ => {}
        }
    }

    fn draw_goal(&self) {
        let pixel = self.layout.hex_to_pixel(self.goal);
        draw_circle(pixel.x, pixel.y, HEXES_SIZE / 2.0, BLACK);
    }

    fn move_player(&mut self, direction: HexDirection) {
        let delta = direction.to_hex();
        if delta == Hex::from_axial(0, 0) {
            return;
        }

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
                if self.hexmap.contains_key(&hex) {
                    PlayerState::Standing(hex)
                } else {
                    PlayerState::Dead
                }
            }
            PlayerState::Flat(head, tail) => {
                if self.hexmap.contains_key(&head) && self.hexmap.contains_key(&tail) {
                    PlayerState::Flat(head, tail)
                } else {
                    PlayerState::Dead
                }
            }
            _ => PlayerState::Dead,
        };
    }

    fn update(&mut self) {
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

    fn draw_hexes(&self, texture: &Texture2D) {
        for (hex, _) in &self.hexmap {
            let pixel = self.layout.hex_to_pixel(*hex) - Vec2::new(25.0, 25.0);
            draw_texture(texture, pixel.x, pixel.y, WHITE);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let pointy: Layout = Layout {
        orientation: Orientation::LAYOUT_POINTY,
        // Point(W/sqrt(3), H/2)
        size: Vec2 {
            x: HEXES_SIZE / SQRT_3,
            y: (32.0) / 2.0,
        },
        origin: Vec2 {
            // x: screen_height() / 2.0,
            // y: screen_width() / 2.0,
            x: 0.0,
            y: 0.0,
        },
    };

    let tmp = HexMap::gen();
    let mut hexmap = tmp.hexmap.clone();
    let player_hex = tmp.start;
    let goal = tmp.goal;

    let mut game = Game {
        layout: pointy,
        hexmap,
        player_state: PlayerState::Standing(player_hex),
        start: player_hex,
        goal: goal,
    };

    let mut game_state = GameState::Playing;

    tmp.solve_path();

    set_pc_assets_folder("assets");
    let texture: Texture2D = load_texture("hex_0.png").await.unwrap();

    // loop {
    //     clear_background(WHITE);

    //     draw_rectangle(0.0, 0.0, SIZE, SIZE, RED);
    //     draw_texture(&texture, 0.0 + 16.0, 0.0, RED);

    //     next_frame().await
    // }

    let mut is_debug = false;
    loop {
        clear_background(WHITE);

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Space) {
                    game.player_state = PlayerState::Standing(player_hex);
                    game_state = GameState::Playing;
                }
                let text = "Press SPACE to start";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    HEXES_SIZE,
                    RED,
                );
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::Y) {
                    is_debug = !is_debug;
                }

                game.update();

                if let PlayerState::Dead = game.player_state {
                    game_state = GameState::GameOver;
                }

                if game.player_state == PlayerState::Standing(game.goal) {
                    game_state = GameState::GameWon;
                }

                game.draw();
                game.draw_hexes(&texture);

                if is_debug {
                    game.draw_ans();
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "GAME OVER!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    HEXES_SIZE,
                    RED,
                );
            }
            GameState::GameWon => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "YOU WON!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    HEXES_SIZE,
                    RED,
                );
            }
        }

        next_frame().await
    }
}
