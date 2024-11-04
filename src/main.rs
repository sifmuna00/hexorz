use std::{collections::HashMap, io::repeat};

use macroquad::prelude::*;

mod hex;

use hex::*;

#[macroquad::main("Super Evil Cats Running Elaborate Trapping Schemes")]
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

    let radius = 4;
    for r in 0..radius {
        let mut h = Hex::from_cube(0, -r, r);
        hexmap.insert(h, 1);

        for dir in 0..6 {
            let num_of_hexas_in_edge = if dir == 5 { r - 1 } else { r };

            for _ in 0..num_of_hexas_in_edge {
                h += Dir::CUBE_DIR[dir];
                hexmap.insert(h, 1);
            }
        }
    }

    let mut player_hex = Hex::from_cube(0, 0, 0);

    loop {
        clear_background(WHITE);

        for (hex, _) in hexmap.iter() {
            let pixel = pointy.hex_to_pixel(*hex);
            draw_hexagon(pixel.x as f32, pixel.y as f32, 50.0, 1.0, true, RED, WHITE);
        }

        let player_pixel = pointy.hex_to_pixel(player_hex);
        draw_hexagon(
            player_pixel.x as f32,
            player_pixel.y as f32,
            25.0,
            1.0,
            true,
            GREEN,
            GREEN,
        );

        next_frame().await
    }
}
