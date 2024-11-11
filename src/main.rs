use std::collections::HashMap;

use macroquad::prelude::*;

mod hex;
use hex::*;

#[derive(Debug, Clone)]
enum PlayerState {
    Standing(Hex),
    Flat(Hex, Hex),
}

struct Player {
    state: PlayerState,
}

impl Player {
    fn draw(&self, layout: &Layout) {
        match self.state {
            PlayerState::Standing(hex) => {
                let pixel = layout.hex_to_pixel(hex);
                draw_circle(pixel.x as f32, pixel.y as f32, 25.0, BLUE);
            }
            PlayerState::Flat(head, tail) => {
                let head_pixel = layout.hex_to_pixel(head);
                let tail_pixel = layout.hex_to_pixel(tail);
                draw_circle(head_pixel.x as f32, head_pixel.y as f32, 25.0, BLUE);
                draw_circle(tail_pixel.x as f32, tail_pixel.y as f32, 25.0, YELLOW);
            }
        }
    }

    fn move_player(&mut self, direction: HexDirection) {
        let delta = direction.to_hex();
        if delta == Hex::from_axial(0, 0) {
            return;
        }

        self.state = match self.state {
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
        size: Vec2 { x: 50.0, y: 50.0 },
        origin: Vec2 {
            x: screen_height() / 2.0,
            y: screen_width() / 2.0,
        },
    };

    let mut hexmap = HashMap::new();
    let mut player_hex = Hex::from_cube(0, 0, 0);

    hexmap.insert(player_hex, 1);
    for hex in player_hex.spiral(3) {
        hexmap.insert(hex, 1);
    }

    let mut main_character = Player {
        state: PlayerState::Standing(player_hex),
    };

    loop {
        clear_background(WHITE);

        for (hex, _) in hexmap.iter() {
            let pixel = pointy.hex_to_pixel(*hex);
            draw_hexagon(pixel.x as f32, pixel.y as f32, 50.0, 1.0, true, RED, WHITE);
        }

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
                main_character.move_player(dir);
            }
        }

        main_character.draw(&pointy);

        next_frame().await
    }
}
