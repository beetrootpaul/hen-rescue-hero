#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BrpColor {
    Solid { r: u8, g: u8, b: u8 },
    Transparent,
}

#[cfg(test)]
pub const fn color_solid(r: u8, g: u8, b: u8) -> BrpColor {
    BrpColor::Solid { r, g, b }
}

impl From<BrpColor> for bevy::render::color::Color {
    fn from(brp_color: BrpColor) -> Self {
        match brp_color {
            BrpColor::Solid { r, g, b } => bevy::render::color::Color::Rgba {
                red: (r as f32) / (0xff as f32),
                green: (g as f32) / (0xff as f32),
                blue: (b as f32) / (0xff as f32),
                alpha: 1.0,
            },
            BrpColor::Transparent => bevy::render::color::Color::BLACK,
        }
    }
}
