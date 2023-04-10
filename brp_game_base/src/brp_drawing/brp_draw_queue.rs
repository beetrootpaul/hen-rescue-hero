use std::collections::VecDeque;

use bevy::math::uvec2;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use brp_drawing::brp_draw::BrpDraw;
use brp_font::{BrpFontConfig, BrpFontGlyph};
use BrpImageAssets;
use BrpSprite;
use {BrpColor, Rect};

pub enum BrpDrawCommand {
    Clear(BrpColor),
    //
    Pixel(IVec2, BrpColor),
    Rect(Rect, BrpColor),
    RectFilled(Rect, BrpColor),
    Ellipse(Rect, BrpColor),
    EllipseFilled(Rect, BrpColor),
    Sprite(IVec2, BrpSprite, bool),
    //
    Text(IVec2, String, BrpColor),
    //
    StartClipping(Rect),
    StopClipping,
}

#[derive(Resource, Default)]
pub struct BrpDrawQueue {
    deque: VecDeque<BrpDrawCommand>,
}

impl BrpDrawQueue {
    pub fn s_draw_queued_commands(
        mut pixels_query: Query<(&bevy_pixels::PixelsOptions, &mut bevy_pixels::PixelsWrapper)>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        brp_image_assets: Res<BrpImageAssets>,
        bevy_image_assets: Res<Assets<Image>>,
        font_config: Res<BrpFontConfig>,
    ) {
        if let Ok((pixels_options, mut pixels_wrapper)) = pixels_query.get_single_mut() {
            let mut draw = BrpDraw {
                canvas_size: uvec2(pixels_options.width, pixels_options.height),
                clipping_rect: None,
            };
            while let Some(draw_command) = draw_queue.deque.pop_front() {
                let frame = pixels_wrapper.pixels.frame_mut();
                match draw_command {
                    BrpDrawCommand::Clear(color) => {
                        draw.clear(frame, color);
                    },
                    //
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
                    BrpDrawCommand::Sprite(
                        xy,
                        BrpSprite {
                            image_path,
                            source_rect,
                            anchor,
                            color_replacements,
                        },
                        flip,
                    ) => {
                        let image_handle = brp_image_assets.get(image_path);
                        let image = bevy_image_assets
                            .get(&image_handle)
                            .expect("should have image for a given handle");
                        draw.draw_sprite(
                            frame,
                            xy - anchor,
                            image.size().x as usize,
                            &image.data,
                            source_rect,
                            color_replacements,
                            flip,
                        );
                    },
                    //
                    BrpDrawCommand::Text(xy, text, text_color) => {
                        if let Some(font_image_path) = font_config.image_path {
                            let font_image_handle = brp_image_assets.get(font_image_path);
                            let font_image = bevy_image_assets
                                .get(&font_image_handle)
                                .expect("should have a font image for a given handle");

                            let mut current_xy = xy;
                            let mut color_replacements: HashMap<BrpColor, BrpColor> =
                                HashMap::new();
                            if let Some(t1) = font_config.source_color_transparent_1 {
                                color_replacements.insert(t1, BrpColor::Transparent);
                            }
                            if let Some(t2) = font_config.source_color_transparent_2 {
                                color_replacements.insert(t2, BrpColor::Transparent);
                            }
                            color_replacements.insert(font_config.source_color_font, text_color);

                            for symbol in text.chars() {
                                let glyph = BrpFontGlyph::of(symbol);
                                if let Some(source_rect) =
                                    font_config.glyph_to_source_rect.get(&glyph)
                                {
                                    draw.draw_sprite(
                                        frame,
                                        current_xy,
                                        font_image.size().x as usize,
                                        &font_image.data,
                                        *source_rect,
                                        color_replacements.clone(),
                                        false,
                                    );
                                    current_xy += font_config.glyph_jump_to_next;
                                }
                            }
                        }
                    },
                    //
                    BrpDrawCommand::StartClipping(clipping_rect) => {
                        draw.clipping_rect = Some(clipping_rect);
                    },
                    BrpDrawCommand::StopClipping => {
                        draw.clipping_rect = None;
                    },
                }
            }
        }
    }

    pub fn enqueue(&mut self, draw_command: BrpDrawCommand) {
        self.deque.push_back(draw_command);
    }
}
