use bevy::utils::HashMap;

use brp_game_base::{rect, BrpFontConfig, BrpFontGlyph};
use images::Images;
use pico8_color::Pico8Color;

pub struct FontEcs;

impl FontEcs {
    pub fn r_font_config() -> BrpFontConfig {
        let glyph_rect = rect(6, 10);
        BrpFontConfig {
            image_path: Some(Images::FONT),
            glyph_size: glyph_rect.size,
            source_color_font: Pico8Color::Red.into(),
            source_color_transparent_1: Some(Pico8Color::LightPeach.into()),
            source_color_transparent_2: Some(Pico8Color::LightGrey.into()),
            glyph_to_source_rect: HashMap::from([
                (BrpFontGlyph::Digit0, glyph_rect.at(1, 3)),
                (BrpFontGlyph::Digit1, glyph_rect.at(9, 3)),
                (BrpFontGlyph::Digit2, glyph_rect.at(17, 3)),
                (BrpFontGlyph::Digit3, glyph_rect.at(25, 3)),
                (BrpFontGlyph::Digit4, glyph_rect.at(33, 3)),
                (BrpFontGlyph::Digit5, glyph_rect.at(41, 3)),
                (BrpFontGlyph::Digit6, glyph_rect.at(49, 3)),
                (BrpFontGlyph::Digit7, glyph_rect.at(57, 3)),
                (BrpFontGlyph::Digit8, glyph_rect.at(65, 3)),
                (BrpFontGlyph::Digit9, glyph_rect.at(73, 3)),
                (BrpFontGlyph::Colon, glyph_rect.at(81, 3)),
            ]),
        }
    }
}
