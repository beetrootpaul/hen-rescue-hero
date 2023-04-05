use bevy::math::{ivec2, IVec2};

#[inline(always)]
pub const fn irect(w: i32, h: i32) -> IRect {
    IRect {
        left_top: IVec2::ZERO,
        size: ivec2(w, h),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IRect {
    pub left_top: IVec2,
    pub size: IVec2,
}

impl IRect {
    pub fn w(self) -> i32 {
        self.size.x
    }
    pub fn h(self) -> i32 {
        self.size.y
    }
    pub fn l(self) -> i32 {
        self.left_top.x
    }
    pub fn t(self) -> i32 {
        self.left_top.y
    }
    pub fn r(self) -> i32 {
        self.left_top.x + self.size.x - 1
    }
    pub fn b(self) -> i32 {
        self.left_top.y + self.size.y - 1
    }

    pub const fn at(self, left: i32, top: i32) -> IRect {
        IRect {
            left_top: ivec2(left, top),
            size: self.size,
        }
    }
}
