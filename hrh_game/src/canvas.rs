use bevy::ecs::system::SystemParam;
use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use brp_game_base::{BrpCanvasVariant, BrpCurrentCanvasVariant, BrpDrawCommand, BrpDrawQueue};
use pico8_color::Pico8Color;
use position::Position;
use sprites::Sprites;

#[derive(SystemParam)]
pub struct Canvas<'w> {
    pub current_canvas_variant: Res<'w, BrpCurrentCanvasVariant>,
}

impl<'w> Canvas<'w> {
    const CANVAS_BORDER: u32 = 1;

    const CANVAS_INNER_TOP_LEFT: IVec2 =
        ivec2(Self::CANVAS_BORDER as i32, Self::CANVAS_BORDER as i32);

    const CANVAS_TILES_LANDSCAPE: UVec2 = uvec2(40, 24);
    const CANVAS_TILES_PORTRAIT: UVec2 = uvec2(24, 36);
    pub const CANVAS_SIZE_LANDSCAPE: UVec2 = uvec2(
        Self::CANVAS_TILES_LANDSCAPE.x * Sprites::TILE_USIZE.x + 2 * Self::CANVAS_BORDER,
        Self::CANVAS_TILES_LANDSCAPE.y * Sprites::TILE_USIZE.y + 2 * Self::CANVAS_BORDER,
    );
    pub const CANVAS_SIZE_PORTRAIT: UVec2 = uvec2(
        Self::CANVAS_TILES_PORTRAIT.x * Sprites::TILE_USIZE.x + 2 * Self::CANVAS_BORDER,
        Self::CANVAS_TILES_PORTRAIT.y * Sprites::TILE_USIZE.y + 2 * Self::CANVAS_BORDER,
    );

    const TOP_BAR_TILES: UVec2 = uvec2(Self::GAME_AREA_TILES.x, 2);

    pub const GAME_AREA_TILES: UVec2 = uvec2(24, 22);
    pub const GAME_AREA_SIZE: UVec2 = uvec2(
        Self::GAME_AREA_TILES.x * Sprites::TILE_USIZE.x,
        Self::GAME_AREA_TILES.y * Sprites::TILE_USIZE.y,
    );

    pub fn canvas_size(&self) -> UVec2 {
        match self.variant() {
            BrpCanvasVariant::Landscape => Self::CANVAS_SIZE_LANDSCAPE,
            BrpCanvasVariant::Portrait => Self::CANVAS_SIZE_PORTRAIT,
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
            left_top: Self::CANVAS_INNER_TOP_LEFT + offset_left * Sprites::TILE_ISIZE,
            size: Self::TOP_BAR_TILES * Sprites::TILE_USIZE,
        }
    }

    pub fn game_area_rect(&self) -> brp_game_base::Rect {
        let top_bar = self.top_bar_rect();
        brp_game_base::Rect {
            left_top: top_bar.left_top + top_bar.size.as_ivec2() * IVec2::Y,
            size: Self::GAME_AREA_SIZE,
        }
    }

    pub fn xy_of_position_within_game_area(&self, game_area_position: Position) -> IVec2 {
        self.game_area_rect().left_top + game_area_position.0.as_ivec2()
    }

    fn variant(&self) -> BrpCanvasVariant {
        self.current_canvas_variant.0.clone()
    }
}

pub struct CanvasEcs;

impl CanvasEcs {
    pub fn s_start_clipping_to_game_area(canvas: Canvas, mut draw_queue: ResMut<BrpDrawQueue>) {
        draw_queue.enqueue(BrpDrawCommand::StartClipping(canvas.game_area_rect()));
    }
    pub fn s_end_clipping_to_game_area(mut draw_queue: ResMut<BrpDrawQueue>) {
        draw_queue.enqueue(BrpDrawCommand::StopClipping);
    }

    pub fn s_draw_bg(canvas: Canvas, mut draw_queue: ResMut<BrpDrawQueue>) {
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
