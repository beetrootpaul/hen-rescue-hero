#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BrpColor {
    Solid { r: u8, g: u8, b: u8 },
    Transparent,
}

#[cfg(test)]
pub const fn color_solid(r: u8, g: u8, b: u8) -> BrpColor {
    BrpColor::Solid { r, g, b }
}
