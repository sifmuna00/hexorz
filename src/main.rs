use ::core::f32;
use macroquad::prelude::*;

mod core;

use core::{game::*, hex::*, map::HexMap};

const HEXES_SIZE: f32 = 32.0;
const MAP_ZOOM: f32 = 2.0;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Playing;
    let mut game = Game::init().await;
    let mut is_debug = false;

    loop {
        clear_background(WHITE);

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Space) {
                    game.player_state = PlayerState::Standing(game.start);
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

                // if let PlayerState::Standing(phex) = game.player_state {
                //     set_camera(&Camera2D {
                //         zoom: vec2(
                //             MAP_ZOOM / screen_width() * 2.0,
                //             MAP_ZOOM / screen_height() * 2.0,
                //         ),
                //         target: pointy.clone().hex_to_pixel(phex),
                //         ..Default::default()
                //     });
                // }

                game.draw(is_debug);
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

fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: 800,
        window_height: 500,
        ..Default::default()
    }
}
