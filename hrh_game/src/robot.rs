use bevy::math::ivec2;
use bevy::prelude::ResMut;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use sprites::Sprites;

pub struct RobotSystems;

impl RobotSystems {
    pub fn draw(mut draw_queue: ResMut<BrpDrawQueue>) {
        draw_queue.enqueue(BrpDrawCommand::Sprite(ivec2(50, 50), Sprites::ROBOT_BODY));
    }
}
