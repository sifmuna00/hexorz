use ::core::f32;
use macroquad::prelude::*;

mod core;

use core::{game::*, hex::*, map::HexMap};

const HEXES_SIZE: f32 = 32.0;

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::MainMenu;
    let mut game = Game::init().await;
    let mut is_debug = false;

    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        // clear_background(WHITE);

        clear_background(BLACK);

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                set_default_camera();

                if is_key_pressed(KeyCode::Space) {
                    game.player_state = PlayerState::Standing(game.map.start);
                    game_state = GameState::Playing;
                }

                let text = "Press SPACE to start";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
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

                if game.player_state == PlayerState::Standing(game.map.goal) {
                    game_state = GameState::GameWon;
                }

                game.draw(is_debug);
            }
            GameState::GameOver => {
                set_default_camera();

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "GAME OVER!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );
            }
            GameState::GameWon => {
                set_default_camera();
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "YOU WON!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    GREEN,
                );
            }
        }

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        ..Default::default()
    }
}
