use bevy::math::{ivec2, uvec2, IVec2, UVec2};

#[inline(always)]
pub const fn rect(w: u32, h: u32) -> Rect {
    Rect {
        left_top: IVec2::ZERO,
        size: uvec2(w, h),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pub left_top: IVec2,
    pub size: UVec2,
}

impl Rect {
    pub fn width(self) -> u32 {
        self.size.x
    }
    pub fn height(self) -> u32 {
        self.size.y
    }
    pub fn top(self) -> i32 {
        self.left_top.y
    }
    pub fn bottom(self) -> i32 {
        self.left_top.y + self.size.y as i32 - 1
    }
    pub fn left(self) -> i32 {
        self.left_top.x
    }
    pub fn right(self) -> i32 {
        self.left_top.x + self.size.x as i32 - 1
    }

    pub const fn at(self, left: i32, top: i32) -> Rect {
        Rect {
            left_top: ivec2(left, top),
            size: self.size,
        }
    }

    pub fn move_by(self, offset: IVec2) -> Rect {
        Rect {
            left_top: self.left_top + offset,
            size: self.size,
        }
    }

    pub fn intersection_with(self, another_rect: Rect) -> Rect {
        let xy0 = self.left_top.clamp(
            another_rect.left_top,
            another_rect.left_top + another_rect.size.as_ivec2(),
        );
        let xy1 = (self.left_top + self.size.as_ivec2()).clamp(
            another_rect.left_top,
            another_rect.left_top + another_rect.size.as_ivec2(),
        );
        assert!(xy1.cmpge(xy0).all());
        Rect {
            left_top: xy0,
            size: (xy1 - xy0).as_uvec2(),
        }
    }
}
