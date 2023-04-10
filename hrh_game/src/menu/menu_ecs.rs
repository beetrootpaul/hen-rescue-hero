use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use animation::Animation;
use brp_game_base::{BrpDrawCommand, BrpDrawQueue, BrpFontConfig, Rect};
use canvas::Canvas;
use game_objects::score::Score;
use input::InputMode;
use menu::menu_arrow_button::MenuArrowButton;
use menu::menu_mode::MenuMode;
use pico8_color::Pico8Color;
use sprite::Sprite;

pub struct MenuEcs;

impl MenuEcs {
    pub fn r_menu_mode() -> MenuMode {
        MenuMode {
            is_first_time: true,
        }
    }

    pub fn s_enter_menu(mut commands: Commands) {
        commands.spawn(MenuArrowButton::new(false));
        commands.spawn(MenuArrowButton::new(true));
    }

    pub fn s_exit_menu(
        q: Query<Entity, With<MenuArrowButton>>,
        mut commands: Commands,
        mut menu_mode: ResMut<MenuMode>,
    ) {
        for button_entity in q.iter() {
            commands.entity(button_entity).despawn_recursive();
        }
        menu_mode.is_first_time = false;
    }

    pub fn s_update(mut q: Query<&mut MenuArrowButton>, time: Res<Time>) {
        for mut button in q.iter_mut() {
            button.advance(time.delta());
        }
    }

    pub fn s_draw(
        canvas: Canvas,
        menu_mode: Res<MenuMode>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        input_mode: Res<InputMode>,
        q_buttons: Query<&MenuArrowButton>,
        score: Res<Score>,
        font_config: Res<BrpFontConfig>,
    ) {
        let mut left_top = canvas.game_area_rect().left_top + ivec2(0, 8) * Sprite::TILE_ISIZE;
        let mut size = uvec2(Canvas::GAME_AREA_TILES.x, 6) * Sprite::TILE_USIZE;
        if !menu_mode.is_first_time {
            left_top += ivec2(0, -2) * Sprite::TILE_ISIZE;
            size += uvec2(0, 4) * Sprite::TILE_USIZE;
        }

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

        if !input_mode.is_input_blocked() {
            for button in q_buttons.iter() {
                let offset_tiles = match (menu_mode.is_first_time, button.is_right) {
                    (true, false) => ivec2(8, 2),
                    (true, true) => ivec2(14, 2),
                    (false, false) => ivec2(8, 6),
                    (false, true) => ivec2(14, 6),
                };
                draw_queue.enqueue(BrpDrawCommand::Sprite(
                    left_top + offset_tiles * Sprite::TILE_ISIZE,
                    button.current_sprite().into(),
                    button.is_right,
                ));
            }
        }

        if !menu_mode.is_first_time {
            let score_text = format!("{}", score.rescued_chickens());
            let score_text_size = font_config.size_of(&score_text);
            let centering_offset = -score_text_size * IVec2::X / 2;

            let nest_xy = left_top
                + size.as_ivec2() / 2
                + centering_offset
                + ivec2(-1, 22)
                + ivec2(0, -4) * Sprite::TILE_ISIZE;
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                nest_xy + ivec2(0, -2),
                Sprite::Chicken.into(),
                false,
            ));
            draw_queue.enqueue(BrpDrawCommand::Sprite(nest_xy, Sprite::Nest.into(), false));
            draw_queue.enqueue(BrpDrawCommand::Text(
                nest_xy + ivec2(11, -10),
                score_text,
                Pico8Color::White.into(),
            ));
        }
    }
}
