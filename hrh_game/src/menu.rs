use bevy::math::{ivec2, uvec2, IVec2, UVec2};
use bevy::prelude::ResMut;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue, Rect};
use canvas::Canvas;
use pico8_color::Pico8Color;
use sprite::Sprite;

pub struct MenuEcs;

impl MenuEcs {
    pub fn s_draw(mut draw_queue: ResMut<BrpDrawQueue>, canvas: Canvas) {
        let left_top = canvas.game_area_rect().left_top + ivec2(0, 8) * Sprite::TILE_ISIZE;
        let size = uvec2(Canvas::GAME_AREA_TILES.x, 6) * Sprite::TILE_USIZE;

        draw_queue.enqueue(BrpDrawCommand::RectFilled(
            Rect { left_top, size },
            Pico8Color::DarkGreen.into(),
        ));
        draw_queue.enqueue(BrpDrawCommand::Rect(
            Rect {
                left_top,
                size: uvec2(size.x, 1),
            },
            Pico8Color::LimeGreen.into(),
        ));
        draw_queue.enqueue(BrpDrawCommand::Rect(
            Rect {
                left_top: left_top + ivec2(0, size.y as i32 - 1),
                size: uvec2(size.x, 1),
            },
            Pico8Color::BlueGreen.into(),
        ));

        draw_queue.enqueue(BrpDrawCommand::Sprite(
            left_top + ivec2(8, 2) * Sprite::TILE_ISIZE,
            Sprite::LeftArrow.into(),
            false,
        ));
        draw_queue.enqueue(BrpDrawCommand::Sprite(
            left_top + ivec2(14, 2) * Sprite::TILE_ISIZE,
            Sprite::LeftArrow.into(),
            true,
        ));
    }
}
