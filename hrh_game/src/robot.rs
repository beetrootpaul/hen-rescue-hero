use bevy::prelude::ResMut;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use images::Images;

pub struct RobotSystems;

impl RobotSystems {
    pub fn draw(mut draw_queue: ResMut<BrpDrawQueue>) {
        draw_queue.enqueue(BrpDrawCommand::Sprite(Images::SPRITE_SHEET))
    }
}
