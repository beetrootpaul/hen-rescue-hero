use bevy::math::{ivec2, uvec2, IVec2, UVec2};
use bevy::utils::HashMap;

use brp_game_base::{BrpSprite, Rect};
use images::Images;
use pico8_color::Pico8Color;

pub enum Sprites {
    RobotBody,
    RobotLeg,
    RobotFace1,
    Chicken,
    Chain,
}

impl Sprites {
    pub const TILE_USIZE: UVec2 = uvec2(8, 8);
    pub const TILE_ISIZE: IVec2 = ivec2(Self::TILE_USIZE.x as i32, Self::TILE_USIZE.y as i32);
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
            Sprites::RobotLeg => s(
                [2, 2, 1, 1],
                [4, 0],
                [
                    (Pico8Color::Yellow, Pico8Color::None),
                    (Pico8Color::Peach, Pico8Color::None),
                ],
            ),
            Sprites::RobotFace1 => s(
                [0, 2, 2, 1],
                [12, 11],
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
            Sprites::Chain => s(
                [2, 3, 1, 1],
                [0, 0],
                [
                    (Pico8Color::Yellow, Pico8Color::None),
                    (Pico8Color::Peach, Pico8Color::None),
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
            left_top: ivec2(x_tile, y_tile) * Sprites::TILE_ISIZE,
            size: ivec2(w_tiles, h_tiles).as_uvec2() * Sprites::TILE_USIZE,
        },
        anchor: ivec2(anchor_x, anchor_y),
        color_replacements: HashMap::from(cr.map(|(c1, c2)| (c1.into(), c2.into()))),
    }
}
