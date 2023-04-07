use bevy::math::{ivec2, uvec2};

use brp_game_base::{BrpSprite, Rect};
use images::Images;

pub struct Sprites;

impl Sprites {
    pub const ROBOT_BODY: BrpSprite = sprite(0, 0, 3, 2);
}

const TILE_W: u32 = 8;
const TILE_H: u32 = 8;

// Param names here are very short in order to have them occupy as minimal space as possible
//   when shown as IDE's inline type hints on a callee side.
#[inline(always)]
const fn sprite(x: i32, y: i32, w: u32, h: u32) -> BrpSprite {
    BrpSprite {
        image_path: Images::SPRITE_SHEET,
        source_rect: Rect {
            left_top: ivec2(x, y),
            size: uvec2(w * TILE_W, h * TILE_H),
        },
    }
}
