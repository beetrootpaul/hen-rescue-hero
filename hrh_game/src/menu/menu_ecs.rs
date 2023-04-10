use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use animation::Animation;
use brp_game_base::{BrpDrawCommand, BrpDrawQueue, Rect};
use canvas::Canvas;
use menu::menu_arrow_button::MenuArrowButton;
use pico8_color::Pico8Color;
use sprite::Sprite;

pub struct MenuEcs;

impl MenuEcs {
    pub fn s_spawn_buttons(mut commands: Commands) {
        commands.spawn(MenuArrowButton::new(false));
        commands.spawn(MenuArrowButton::new(true));
    }

    pub fn s_despawn_buttons(q: Query<Entity, With<MenuArrowButton>>, mut commands: Commands) {
        for button_entity in q.iter() {
            commands.entity(button_entity).despawn_recursive();
        }
    }

    pub fn s_update(mut q: Query<&mut MenuArrowButton>, time: Res<Time>) {
        for mut button in q.iter_mut() {
            button.advance(time.delta());
        }
    }

    pub fn s_draw(
        canvas: Canvas,
        mut draw_queue: ResMut<BrpDrawQueue>,
        q_buttons: Query<&MenuArrowButton>,
    ) {
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

        for button in q_buttons.iter() {
            let offset_tiles = match button.is_right {
                false => ivec2(8, 2),
                true => ivec2(14, 2),
            };
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                left_top + offset_tiles * Sprite::TILE_ISIZE,
                button.current_sprite().into(),
                button.is_right,
            ));
        }
    }
}
