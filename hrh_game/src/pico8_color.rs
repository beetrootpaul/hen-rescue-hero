use brp_game_base::BrpColor;

pub enum Pico8Color {
    None,

    // main 16 colors
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGrey,
    LightGrey,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Lavender,
    Pink,
    LightPeach,

    // additional "secret" 16 colors
    BrownishBlack,
    DarkerBlue,
    DarkerPurple,
    BlueGreen,
    DarkBrown,
    DarkerGrey,
    MediumGrey,
    LightYellow,
    DarkRed,
    DarkOrange,
    LimeGreen,
    MediumGreen,
    TrueBlue,
    Mauve,
    DarkPeach,
    Peach,
}

impl Pico8Color {
    fn hex(&self) -> Option<&str> {
        match *self {
            Pico8Color::None => None,

            // hex values taken from https://pico-8.fandom.com/wiki/Palette#The_system_palette

            // main 16 colors
            Pico8Color::Black => Some("000000"),
            Pico8Color::DarkBlue => Some("1d2b53"),
            Pico8Color::DarkPurple => Some("7e2553"),
            Pico8Color::DarkGreen => Some("008751"),
            Pico8Color::Brown => Some("ab5236"),
            Pico8Color::DarkGrey => Some("5f574f"),
            Pico8Color::LightGrey => Some("c2c3c7"),
            Pico8Color::White => Some("fff1e8"),
            Pico8Color::Red => Some("ff004d"),
            Pico8Color::Orange => Some("ffa300"),
            Pico8Color::Yellow => Some("ffec27"),
            Pico8Color::Green => Some("00e436"),
            Pico8Color::Blue => Some("29adff"),
            Pico8Color::Lavender => Some("83769c"),
            Pico8Color::Pink => Some("ff77a8"),
            Pico8Color::LightPeach => Some("ffccaa"),

            // additional "secret" 16 colors
            Pico8Color::BrownishBlack => Some("291814"),
            Pico8Color::DarkerBlue => Some("111d35"),
            Pico8Color::DarkerPurple => Some("422136"),
            Pico8Color::BlueGreen => Some("125359"),
            Pico8Color::DarkBrown => Some("742f29"),
            Pico8Color::DarkerGrey => Some("49333b"),
            Pico8Color::MediumGrey => Some("a28879"),
            Pico8Color::LightYellow => Some("f3ef7d"),
            Pico8Color::DarkRed => Some("be1250"),
            Pico8Color::DarkOrange => Some("ff6c24"),
            Pico8Color::LimeGreen => Some("a8e72e"),
            Pico8Color::MediumGreen => Some("00b543"),
            Pico8Color::TrueBlue => Some("065ab5"),
            Pico8Color::Mauve => Some("754665"),
            Pico8Color::DarkPeach => Some("ff6e59"),
            Pico8Color::Peach => Some("ff9d81"),
        }
    }

    fn rgb8(&self) -> Option<(u8, u8, u8)> {
        self.hex().map(|hex| {
            (
                u8::from_str_radix(&hex[0..2], 16)
                    .expect("should convert from string hex to number"),
                u8::from_str_radix(&hex[2..4], 16)
                    .expect("should convert from string hex to number"),
                u8::from_str_radix(&hex[4..6], 16)
                    .expect("should convert from string hex to number"),
            )
        })
    }
}

impl From<Pico8Color> for BrpColor {
    fn from(pico8_color: Pico8Color) -> Self {
        match pico8_color.rgb8() {
            Some((r, g, b)) => BrpColor::Solid { r, g, b },
            None => BrpColor::Transparent,
        }
    }
}
