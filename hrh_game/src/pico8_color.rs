use brp_game_base::BrpColor;

pub enum Pico8Color {
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
    fn hex(&self) -> &str {
        match *self {
            // hex values taken from https://pico-8.fandom.com/wiki/Palette#The_system_palette

            // main 16 colors
            Pico8Color::Black => "000000",
            Pico8Color::DarkBlue => "1d2b53",
            Pico8Color::DarkPurple => "7e2553",
            Pico8Color::DarkGreen => "008751",
            Pico8Color::Brown => "ab5236",
            Pico8Color::DarkGrey => "5f574f",
            Pico8Color::LightGrey => "c2c3c7",
            Pico8Color::White => "fff1e8",
            Pico8Color::Red => "ff004d",
            Pico8Color::Orange => "ffa300",
            Pico8Color::Yellow => "ffec27",
            Pico8Color::Green => "00e436",
            Pico8Color::Blue => "29adff",
            Pico8Color::Lavender => "83769c",
            Pico8Color::Pink => "ff77a8",
            Pico8Color::LightPeach => "ffccaa",

            // additional "secret" 16 colors
            Pico8Color::BrownishBlack => "291814",
            Pico8Color::DarkerBlue => "111d35",
            Pico8Color::DarkerPurple => "422136",
            Pico8Color::BlueGreen => "125359",
            Pico8Color::DarkBrown => "742f29",
            Pico8Color::DarkerGrey => "49333b",
            Pico8Color::MediumGrey => "a28879",
            Pico8Color::LightYellow => "f3ef7d",
            Pico8Color::DarkRed => "be1250",
            Pico8Color::DarkOrange => "ff6c24",
            Pico8Color::LimeGreen => "a8e72e",
            Pico8Color::MediumGreen => "00b543",
            Pico8Color::TrueBlue => "065ab5",
            Pico8Color::Mauve => "754665",
            Pico8Color::DarkPeach => "ff6e59",
            Pico8Color::Peach => "ff9d81",
        }
    }

    fn rgb8(&self) -> (u8, u8, u8) {
        let hex = self.hex();
        (
            u8::from_str_radix(&hex[0..2], 16).expect("should convert from string hex to number"),
            u8::from_str_radix(&hex[2..4], 16).expect("should convert from string hex to number"),
            u8::from_str_radix(&hex[4..6], 16).expect("should convert from string hex to number"),
        )
    }
}

impl From<Pico8Color> for BrpColor {
    fn from(pico8_color: Pico8Color) -> Self {
        let (r, g, b) = pico8_color.rgb8();
        BrpColor::Solid { r, g, b }
    }
}
