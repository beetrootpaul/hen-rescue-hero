use bevy::math::{ivec2, uvec2, IVec2, UVec2};
use bevy::utils::HashMap;

use brp_game_base::{BrpSprite, Rect};
use images::Images;
use pico8_color::Pico8Color;

pub enum Sprite {
    RobotBody,
    RobotLeg,
    RobotFace1,
    RobotFace2,
    RobotFace3,
    RobotFace5,
    //
    Chain,
    //
    Chicken,
    //
    PileOfChicken1,
    PileOfChicken2,
    PileOfChicken3,
    PileOfChicken4,
    PileOfChicken5,
    PileOfChicken6,
    PileOfChicken7,
    PileOfChicken8,
    //
    Side,
    Nest,
}

impl Sprite {
    pub const TILE_USIZE: UVec2 = uvec2(8, 8);
    pub const TILE_ISIZE: IVec2 = ivec2(Self::TILE_USIZE.x as i32, Self::TILE_USIZE.y as i32);

    const COLOR_REPLACEMENTS_1: [(Pico8Color, Pico8Color); 2] = [
        (Pico8Color::Yellow, Pico8Color::None),
        (Pico8Color::Peach, Pico8Color::None),
    ];
    const COLOR_REPLACEMENTS_2: [(Pico8Color, Pico8Color); 2] = [
        (Pico8Color::Yellow, Pico8Color::None),
        (Pico8Color::LimeGreen, Pico8Color::None),
    ];
}

impl From<Sprite> for BrpSprite {
    fn from(sprite: Sprite) -> Self {
        match sprite {
            Sprite::RobotBody => s([0, 0, 3, 2], [12, 16], Sprite::COLOR_REPLACEMENTS_1),
            Sprite::RobotLeg => s([2, 2, 1, 1], [4, 0], Sprite::COLOR_REPLACEMENTS_1),
            Sprite::RobotFace1 => s([0, 2, 2, 1], [12, 11], Sprite::COLOR_REPLACEMENTS_1),
            Sprite::RobotFace2 => s([0, 3, 2, 1], [12, 11], Sprite::COLOR_REPLACEMENTS_1),
            Sprite::RobotFace3 => s([0, 4, 2, 1], [12, 11], Sprite::COLOR_REPLACEMENTS_1),
            Sprite::RobotFace5 => s([0, 6, 2, 1], [12, 11], Sprite::COLOR_REPLACEMENTS_1),
            //
            Sprite::Chain => s([2, 3, 1, 1], [0, 0], Sprite::COLOR_REPLACEMENTS_1),
            //
            Sprite::Chicken => s([3, 0, 2, 2], [8, 16], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken1 => s([3, 2, 2, 2], [7, 16 + 11], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken2 => s([3, 4, 2, 2], [7, 16 + 11], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken3 => s([3, 6, 2, 3], [7, 24 + 11], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken4 => s([3, 9, 2, 3], [7, 24 + 11], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken5 => s([5, 0, 2, 3], [7, 24 + 11], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken6 => s([5, 3, 2, 4], [7, 32 + 11], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken7 => s([5, 7, 2, 4], [7, 32 + 11], Sprite::COLOR_REPLACEMENTS_2),
            Sprite::PileOfChicken8 => s([7, 0, 2, 4], [7, 32 + 11], Sprite::COLOR_REPLACEMENTS_2),
            //
            Sprite::Side => s([0, 9, 2, 3], [8, 24], Sprite::COLOR_REPLACEMENTS_1),
            Sprite::Nest => s([0, 8, 2, 1], [8, 6], Sprite::COLOR_REPLACEMENTS_1),
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
            left_top: ivec2(x_tile, y_tile) * Sprite::TILE_ISIZE,
            size: ivec2(w_tiles, h_tiles).as_uvec2() * Sprite::TILE_USIZE,
        },
        anchor: ivec2(anchor_x, anchor_y),
        color_replacements: HashMap::from(cr.map(|(c1, c2)| (c1.into(), c2.into()))),
    }
}
