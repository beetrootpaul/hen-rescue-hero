use bevy::ecs::system::SystemParam;
use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use brp_game_base::{BrpCanvasVariant, BrpCurrentCanvasVariant, BrpDrawCommand, BrpDrawQueue};
use pico8_color::Pico8Color;
use sprites::TILE_SIZE;

const CANVAS_BORDER: u32 = 1;
const CANVAS_INNER_TOP_LEFT: IVec2 = ivec2(CANVAS_BORDER as i32, CANVAS_BORDER as i32);
const CANVAS_TILES_LANDSCAPE: UVec2 = uvec2(40, 24);
const CANVAS_TILES_PORTRAIT: UVec2 = uvec2(24, 36);
const GAME_AREA_TILES: UVec2 = uvec2(24, 22);
const TOP_BAR_TILES: UVec2 = uvec2(GAME_AREA_TILES.x, 2);

#[derive(SystemParam)]
pub struct Canvas<'w> {
    pub current_canvas_variant: Res<'w, BrpCurrentCanvasVariant>,
}

impl<'w> Canvas<'w> {
    pub const fn canvas_size_landscape() -> UVec2 {
        uvec2(
            CANVAS_TILES_LANDSCAPE.x * TILE_SIZE.x + 2 * CANVAS_BORDER,
            CANVAS_TILES_LANDSCAPE.y * TILE_SIZE.y + 2 * CANVAS_BORDER,
        )
    }
    pub const fn canvas_size_portrait() -> UVec2 {
        uvec2(
            CANVAS_TILES_PORTRAIT.x * TILE_SIZE.x + 2 * CANVAS_BORDER,
            CANVAS_TILES_PORTRAIT.y * TILE_SIZE.y + 2 * CANVAS_BORDER,
        )
    }
    pub const fn game_area_size() -> UVec2 {
        uvec2(
            GAME_AREA_TILES.x * TILE_SIZE.x,
            GAME_AREA_TILES.y * TILE_SIZE.y,
        )
    }

    pub fn canvas_size(&self) -> UVec2 {
        match self.variant() {
            BrpCanvasVariant::Landscape => Self::canvas_size_landscape(),
            BrpCanvasVariant::Portrait => Self::canvas_size_portrait(),
        }
    }

    pub fn border_rect(&self) -> brp_game_base::Rect {
        brp_game_base::Rect {
            left_top: IVec2::ZERO,
            size: self.canvas_size(),
        }
    }

    pub fn top_bar_rect(&self) -> brp_game_base::Rect {
        let offset_left = match self.variant() {
            BrpCanvasVariant::Landscape => ivec2(8, 0),
            BrpCanvasVariant::Portrait => ivec2(0, 0),
        };
        brp_game_base::Rect {
            left_top: CANVAS_INNER_TOP_LEFT + offset_left * TILE_SIZE.as_ivec2(),
            size: TOP_BAR_TILES * TILE_SIZE,
        }
    }

    pub fn game_area_rect(&self) -> brp_game_base::Rect {
        let top_bar = self.top_bar_rect();
        brp_game_base::Rect {
            left_top: top_bar.left_top + top_bar.size.as_ivec2() * IVec2::Y,
            size: Self::game_area_size(),
        }
    }

    fn variant(&self) -> BrpCanvasVariant {
        self.current_canvas_variant.0.clone()
    }
}

pub struct CanvasSystems;

impl CanvasSystems {
    pub fn draw_bg(canvas: Canvas, mut draw_queue: ResMut<BrpDrawQueue>) {
        draw_queue.enqueue(BrpDrawCommand::Clear(Pico8Color::BrownishBlack.into()));
        draw_queue.enqueue(BrpDrawCommand::RectFilled(
            canvas.game_area_rect(),
            Pico8Color::Blue.into(),
        ));
        draw_queue.enqueue(BrpDrawCommand::RectFilled(
            canvas.top_bar_rect(),
            Pico8Color::BrownishBlack.into(),
        ));
        draw_queue.enqueue(BrpDrawCommand::Rect(
            canvas.border_rect(),
            Pico8Color::LightPeach.into(),
        ));
    }
}
