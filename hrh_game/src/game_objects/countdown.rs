use std::time::Duration;

use bevy::math::ivec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue, BrpFontConfig, BrpGameState};
use canvas::Canvas;
use input::InputMode;
use pico8_color::Pico8Color;
use sprite::Sprite;

#[derive(Resource)]
pub struct Countdown {
    timer: Timer,
}

impl Countdown {
    pub fn advance_by(&mut self, delta_time: Duration) {
        self.timer.tick(delta_time);
    }
    pub fn remaining_seconds(&self) -> u32 {
        self.timer.remaining().as_secs_f32().ceil() as u32
    }
    pub fn just_finished(&self) -> bool {
        self.timer.just_finished()
    }
    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

pub struct CountdownEcs;

impl CountdownEcs {
    pub fn r_countdown() -> Countdown {
        Countdown {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }

    pub fn s_reset(mut countdown: ResMut<Countdown>) {
        countdown.reset();
    }

    pub fn s_update(
        mut countdown: ResMut<Countdown>,
        time: Res<Time>,
        mut next_state: ResMut<NextState<BrpGameState>>,
        mut input_mode: ResMut<InputMode>,
    ) {
        countdown.advance_by(time.delta());
        if countdown.just_finished() {
            next_state.set(BrpGameState::Menu);
            input_mode.block_input_for_a_moment();
        }
    }

    pub fn s_draw(
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
        countdown: Res<Countdown>,
        font_config: Res<BrpFontConfig>,
    ) {
        let top_bar_rect = canvas.top_bar_rect();
        let clock_xy = ivec2(
            top_bar_rect.left_top.x + top_bar_rect.size.x as i32,
            top_bar_rect.left_top.y,
        ) + ivec2(-4, 14);

        draw_queue.enqueue(BrpDrawCommand::Sprite(
            clock_xy,
            Sprite::Clock.into(),
            false,
        ));

        let countdown_text = format!("{}", countdown.remaining_seconds());
        let countdown_text_rect = font_config.rect_of(&countdown_text);
        draw_queue.enqueue(BrpDrawCommand::Text(
            clock_xy - ivec2(countdown_text_rect.x + 13, 11),
            countdown_text,
            Pico8Color::LightPeach.into(),
        ));
    }
}
