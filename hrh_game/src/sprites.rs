use bevy::math::{ivec2, uvec2, IVec2, UVec2};
use bevy::utils::HashMap;

use brp_game_base::{BrpSprite, Rect};
use images::Images;
use pico8_color::Pico8Color;

pub const TILE_SIZE: UVec2 = uvec2(8, 8);

pub enum Sprites {
    RobotBody,
    Chicken,
}

impl From<Sprites> for BrpSprite {
    fn from(sprite: Sprites) -> Self {
        match sprite {
            Sprites::RobotBody => s(
                [0, 0, 3, 2],
                [12, 16],
                [
                    (Pico8Color::Yellow, Pico8Color::None),
                    (Pico8Color::Peach, Pico8Color::None),
                ],
            ),
            Sprites::Chicken => s(
                [3, 0, 2, 2],
                [8, 16],
                [
                    (Pico8Color::Yellow, Pico8Color::None),
                    (Pico8Color::LimeGreen, Pico8Color::None),
                ],
            ),
        }
    }
}

// Param names and structure here are very designed to take small amount of space in IDE on a callee side,
//   both in terms of actual typed characters, as well as IDE's inline type hints.
fn s<const N: usize>(xywh: [i32; 4], a: [i32; 2], cr: [(Pico8Color, Pico8Color); N]) -> BrpSprite {
    let [x_tile, y_tile, w_tiles, h_tiles] = xywh;
    let [anchor_x, anchor_y] = a;
    BrpSprite {
        image_path: Images::SPRITE_SHEET,
        source_rect: Rect {
            left_top: ivec2(x_tile, y_tile) * TILE_SIZE.as_ivec2(),
            size: ivec2(w_tiles, h_tiles).as_uvec2() * TILE_SIZE,
        },
        anchor: ivec2(anchor_x, anchor_y),
        color_replacements: HashMap::from(cr.map(|(c1, c2)| (c1.into(), c2.into()))),
    }
}
