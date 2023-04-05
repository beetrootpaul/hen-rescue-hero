use std::collections::VecDeque;

use bevy::math::uvec2;
use bevy::prelude::*;

use drawing::draw::BrpDraw;
use {BrpColor, IRect};

pub enum BrpDrawCommand {
    Clear(BrpColor),
    Pixel(IVec2, BrpColor),
    Rect(IRect, BrpColor),
    RectFilled(IRect, BrpColor),
    Ellipse(IRect, BrpColor),
    EllipseFilled(IRect, BrpColor),
}

#[derive(Resource, Default)]
pub struct BrpDrawQueue {
    deque: VecDeque<BrpDrawCommand>,
}

impl BrpDrawQueue {
    pub fn sys_draw_queued_commands(
        mut pixels_query: Query<(&bevy_pixels::PixelsOptions, &mut bevy_pixels::PixelsWrapper)>,
        mut draw_queue: ResMut<BrpDrawQueue>,
    ) {
        if let Ok((pixels_options, mut pixels_wrapper)) = pixels_query.get_single_mut() {
            let draw = BrpDraw {
                canvas_size: uvec2(pixels_options.width, pixels_options.height),
            };
            while let Some(draw_command) = draw_queue.deque.pop_front() {
                let frame = pixels_wrapper.pixels.frame_mut();
                match draw_command {
                    BrpDrawCommand::Clear(color) => {
                        draw.clear(frame, color);
                    },
                    BrpDrawCommand::Pixel(xy, color) => {
                        draw.draw_pixel(frame, xy, color);
                    },
                    BrpDrawCommand::Rect(rect, color) => {
                        draw.draw_rect(frame, rect, color, false);
                    },
                    BrpDrawCommand::RectFilled(rect, color) => {
                        draw.draw_rect(frame, rect, color, true);
                    },
                    BrpDrawCommand::Ellipse(bounding_rect, color) => {
                        draw.draw_ellipse(frame, bounding_rect, color, false);
                    },
                    BrpDrawCommand::EllipseFilled(bounding_rect, color) => {
                        draw.draw_ellipse(frame, bounding_rect, color, true);
                    },
                }
            }
        }
    }

    pub fn enqueue(&mut self, draw_command: BrpDrawCommand) {
        self.deque.push_back(draw_command);
    }
}