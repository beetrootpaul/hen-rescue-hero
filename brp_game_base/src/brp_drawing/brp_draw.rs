use bevy::math::{ivec2, IVec2, UVec2};
use bevy::render::texture::Image;

use rect;
use {BrpColor, Rect};

// amount of bytes per pixel in `pixels` crate's frame buffer
const PX_LEN: usize = 4;

pub struct BrpDraw {
    pub canvas_size: UVec2,
}

impl BrpDraw {
    pub fn clear(&self, frame: &mut [u8], color: BrpColor) {
        if let BrpColor::Solid { r, g, b } = color {
            frame.copy_from_slice(&[r, g, b, 0xff].repeat(frame.len() / PX_LEN));
        }
    }

    pub fn draw_pixel(&self, frame: &mut [u8], xy: IVec2, color: BrpColor) {
        self.set_pixel(frame, xy, color);
    }

    pub fn draw_rect(&self, frame: &mut [u8], rect: Rect, color: BrpColor, fill: bool) {
        for y in rect.top()..=rect.bottom() {
            if fill || y == rect.top() || y == rect.bottom() {
                self.set_pixels_x0x1(frame, ivec2(rect.left_top.x, y), rect.right() + 1, color);
            } else {
                self.set_pixel(frame, ivec2(rect.left(), y), color);
                self.set_pixel(frame, ivec2(rect.right(), y), color);
            }
        }
    }

    // Based on https://github.com/aseprite/aseprite/blob/25fbe786f8353a2ddb57de3bcc5db00066cc9ca6/src/doc/algo.cpp#L216-L315 (license: MIT)
    pub fn draw_ellipse(&self, frame: &mut [u8], bounding_rect: Rect, color: BrpColor, fill: bool) {
        if let BrpColor::Transparent = color {
            return;
        }

        assert!(bounding_rect.width() > 0);
        assert!(bounding_rect.height() > 0);

        let mut x0: i32 = bounding_rect.left();
        let mut x1: i32 = bounding_rect.right();
        let mut y0: i32 = bounding_rect.top();
        let mut y1: i32;

        let h: i32 = bounding_rect.height() as i32;

        let mut a: i32 = bounding_rect.width() as i32 - 1;
        let b: i32 = bounding_rect.height() as i32 - 1;
        let mut b1: i32 = b & 1;

        let mut dx: i32 = 4 * (1 - a) * b * b;
        let mut dy: i32 = 4 * (b1 + 1) * a * a;

        let mut err: i32 = dx + dy + (b1 * a * a);
        let mut err2: i32;

        y0 += (b + 1) / 2;
        y1 = y0 - b1;
        a = 8 * a * a;
        b1 = 8 * b * b;

        loop {
            if fill {
                self.set_pixels_x0x1(frame, ivec2(x0, y0), x1 + 1, color);
                self.set_pixels_x0x1(frame, ivec2(x0, y1), x1 + 1, color);
            } else {
                self.set_pixel(frame, ivec2(x1, y0), color);
                self.set_pixel(frame, ivec2(x0, y0), color);
                self.set_pixel(frame, ivec2(x0, y1), color);
                self.set_pixel(frame, ivec2(x1, y1), color);
            }

            err2 = 2 * err;
            if err2 <= dy {
                y0 += 1;
                y1 -= 1;
                dy += a;
                err += dy;
            }
            if err2 >= dx || 2 * err > dy {
                x0 += 1;
                x1 -= 1;
                dx += b1;
                err += dx;
            }

            if x0 > x1 {
                break;
            }
        }

        while y0 - y1 < h {
            self.set_pixel(frame, ivec2(x0 - 1, y0), color);
            self.set_pixel(frame, ivec2(x1 + 1, y0), color);
            y0 += 1;
            self.set_pixel(frame, ivec2(x0 - 1, y1), color);
            self.set_pixel(frame, ivec2(x1 + 1, y1), color);
            y1 -= 1;
        }
    }

    pub fn draw_sprite(&self, frame: &mut [u8], img: &Image) {
        let target_xy = ivec2(16, 16);
        let source_rect = rect(30, 30).at(4, 4);

        if let Some(pixel_index) = self.frame_index_of(target_xy) {
            let sprite_w = source_rect.width() as usize;
            let sprite_h = source_rect.height() as usize;

            let sprite_bytes: &[u8] = &img.data;

            for sprite_row in 0..sprite_h {
                for sprite_column in 0..sprite_w {
                    let target_i = pixel_index
                        + (sprite_row * (self.canvas_size.x as usize) + sprite_column) * PX_LEN;
                    let source_i = ((source_rect.top() as usize + sprite_row)
                        * (img.size().x as usize)
                        + (source_rect.left() as usize + sprite_column))
                        * PX_LEN;
                    let source_rgba = &sprite_bytes[source_i..(source_i + PX_LEN)];

                    frame[target_i] = source_rgba[0];
                    frame[target_i + 1] = source_rgba[1];
                    frame[target_i + 2] = source_rgba[2];
                    frame[target_i + 3] = source_rgba[3];
                }
            }
        }
    }

    fn set_pixel(&self, frame: &mut [u8], xy: IVec2, color: BrpColor) {
        if let BrpColor::Solid { r, g, b } = color {
            if let Some(idx) = self.frame_index_of(xy) {
                frame[idx..(idx + PX_LEN)].copy_from_slice(&[r, g, b, 0xff]);
            }
        }
    }

    pub fn set_pixels_x0x1(&self, frame: &mut [u8], xy0: IVec2, x1: i32, color: BrpColor) {
        assert!(x1 >= xy0.x);

        if let BrpColor::Solid { r, g, b } = color {
            let left = ivec2(xy0.x.clamp(0, self.canvas_size.x as i32), xy0.y);
            let right = ivec2(x1.clamp(0, self.canvas_size.x as i32), left.y) - ivec2(1, 0);

            if let (Some(idx1), Some(idx2)) =
                (self.frame_index_of(left), self.frame_index_of(right))
            {
                let idx2 = idx2 + PX_LEN;
                frame[idx1..idx2].copy_from_slice(&[r, g, b, 0xff].repeat((idx2 - idx1) / PX_LEN));
            }
        }
    }

    fn frame_index_of(&self, xy: IVec2) -> Option<usize> {
        let canvas_size = self.canvas_size.as_ivec2();
        if xy.cmpge(IVec2::ZERO).all() && xy.cmplt(canvas_size).all() {
            Some((xy.y * canvas_size.x + xy.x) as usize * PX_LEN)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use bevy::math::{ivec2, uvec2};
    use bevy::prelude::IVec2;
    use bevy::utils::HashMap;

    use brp_color::{color_solid, BrpColor};
    use rect;

    use super::*;

    #[test]
    fn test_clear_frame() {
        let mut h = TestHelper::for_canvas_size(3, 3);
        let color_bg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);

        h.assert_frame_pixels(
            vec![("#", color_bg)],
            "
                ###
                ###
                ###
            ",
        );
    }

    #[test]
    fn test_draw_pixel() {
        let mut h = TestHelper::for_canvas_size(3, 3);
        let color_bg = color_solid(9, 8, 7);
        let color_1 = color_solid(1, 2, 3);
        let color_2 = color_solid(4, 5, 6);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw.draw_pixel(&mut h.frame, ivec2(0, 0), color_1);
        h.draw.draw_pixel(&mut h.frame, ivec2(2, 2), color_2);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_1), ("@", color_2)],
            "
                #--
                ---
                --@
            ",
        );
    }

    #[test]
    fn test_draw_pixel_outside_canvas() {
        let mut h = TestHelper::for_canvas_size(3, 3);
        let color_bg = color_solid(9, 8, 7);
        let color_1 = color_solid(1, 2, 3);
        let color_2 = color_solid(4, 5, 6);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw.draw_pixel(&mut h.frame, ivec2(-1, 2), color_1);
        h.draw.draw_pixel(&mut h.frame, ivec2(1, 3), color_2);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_1), ("@", color_2)],
            "
                ---
                ---
                ---
            ",
        );
    }

    #[test]
    fn test_draw_rect_5x4() {
        let mut h = TestHelper::for_canvas_size(7, 6);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_rect(&mut h.frame, rect(5, 4).at(1, 1), color_fg, false);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                -------
                -#####-
                -#---#-
                -#---#-
                -#####-
                -------
            ",
        );
    }

    #[test]
    fn test_draw_rect_filled_5x4() {
        let mut h = TestHelper::for_canvas_size(7, 6);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_rect(&mut h.frame, rect(5, 4).at(1, 1), color_fg, true);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                -------
                -#####-
                -#####-
                -#####-
                -#####-
                -------
            ",
        );
    }

    #[test]
    fn test_draw_rects_clipped() {
        let mut h = TestHelper::for_canvas_size(5, 5);
        let color_bg = color_solid(9, 8, 7);
        let color_1 = color_solid(1, 1, 1);
        let color_2 = color_solid(2, 2, 2);
        let color_3 = color_solid(3, 3, 3);
        let color_4 = color_solid(4, 4, 4);
        let color_5 = color_solid(5, 5, 5);

        h.draw.clear(&mut h.frame, color_bg);
        // clipped from the left
        h.draw
            .draw_rect(&mut h.frame, rect(3, 3).at(-1, 1), color_1, true);
        // clipped from the right
        h.draw
            .draw_rect(&mut h.frame, rect(3, 3).at(3, 1), color_2, true);
        // clipped from the top
        h.draw
            .draw_rect(&mut h.frame, rect(3, 3).at(1, -1), color_3, true);
        // clipped from the bottom
        h.draw
            .draw_rect(&mut h.frame, rect(3, 3).at(1, 3), color_4, true);
        // drawn last, but clipped entirely
        h.draw
            .draw_rect(&mut h.frame, rect(3, 3).at(-3, 0), color_5, true);

        h.assert_frame_pixels(
            vec![
                ("-", color_bg),
                ("#", color_1),
                ("@", color_2),
                ("%", color_3),
                ("*", color_4),
                ("!", color_5),
            ],
            "
                -%%%-
                #%%%@
                ##-@@
                #***@
                -***-
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_1x1() {
        let mut h = TestHelper::for_canvas_size(3, 3);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_ellipse(&mut h.frame, rect(1, 1).at(1, 1), color_fg, false);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                ---
                -#-
                ---
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_filled_1x1() {
        let mut h = TestHelper::for_canvas_size(3, 3);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_ellipse(&mut h.frame, rect(1, 1).at(1, 1), color_fg, true);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                ---
                -#-
                ---
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_2x2() {
        let mut h = TestHelper::for_canvas_size(4, 4);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_ellipse(&mut h.frame, rect(2, 2).at(1, 1), color_fg, false);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                ----
                -##-
                -##-
                ----
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_filled_2x2() {
        let mut h = TestHelper::for_canvas_size(4, 4);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_ellipse(&mut h.frame, rect(2, 2).at(1, 1), color_fg, true);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                ----
                -##-
                -##-
                ----
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_12x5() {
        let mut h = TestHelper::for_canvas_size(14, 7);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_ellipse(&mut h.frame, rect(12, 5).at(1, 1), color_fg, false);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                --------------
                ----######----
                --##------##--
                -#----------#-
                --##------##--
                ----######----
                --------------
            ",
        );
    }

    #[test]
    fn test_draw_ellipse_filled_12x5() {
        let mut h = TestHelper::for_canvas_size(14, 7);
        let color_bg = color_solid(9, 8, 7);
        let color_fg = color_solid(1, 2, 3);

        h.draw.clear(&mut h.frame, color_bg);
        h.draw
            .draw_ellipse(&mut h.frame, rect(12, 5).at(1, 1), color_fg, true);

        h.assert_frame_pixels(
            vec![("-", color_bg), ("#", color_fg)],
            "
                --------------
                ----######----
                --##########--
                -############-
                --##########--
                ----######----
                --------------
            ",
        );
    }

    #[test]
    fn test_draw_ellipses_clipped() {
        let mut h = TestHelper::for_canvas_size(5, 5);
        let color_bg = color_solid(9, 8, 7);
        let color_1 = color_solid(1, 1, 1);
        let color_2 = color_solid(2, 2, 2);
        let color_3 = color_solid(3, 3, 3);
        let color_4 = color_solid(4, 4, 4);
        let color_5 = color_solid(5, 5, 5);

        h.draw.clear(&mut h.frame, color_bg);
        // clipped from the left
        h.draw
            .draw_ellipse(&mut h.frame, rect(3, 3).at(-1, 1), color_1, false);
        // clipped from the right
        h.draw
            .draw_ellipse(&mut h.frame, rect(3, 3).at(3, 1), color_2, false);
        // clipped from the top
        h.draw
            .draw_ellipse(&mut h.frame, rect(3, 3).at(1, -1), color_3, false);
        // clipped from the bottom
        h.draw
            .draw_ellipse(&mut h.frame, rect(3, 3).at(1, 3), color_4, false);
        // drawn last, but clipped entirely
        h.draw
            .draw_ellipse(&mut h.frame, rect(3, 3).at(-2, -2), color_5, false);

        h.assert_frame_pixels(
            vec![
                ("-", color_bg),
                ("#", color_1),
                ("@", color_2),
                ("%", color_3),
                ("*", color_4),
                ("!", color_5),
            ],
            "
                -%-%-
                #-%-@
                -#-@-
                #-*-@
                -*-*-
            ",
        );
    }

    struct TestHelper {
        canvas_size: UVec2,
        frame: Vec<u8>,
        draw: BrpDraw,
    }

    impl TestHelper {
        fn for_canvas_size(w: u32, h: u32) -> Self {
            let canvas_size = uvec2(w, h);
            Self {
                canvas_size,
                frame: vec![0; PX_LEN * (w as usize) * (h as usize)],
                draw: BrpDraw { canvas_size },
            }
        }

        fn assert_frame_pixels(
            &self,
            color_symbols: Vec<(&str, BrpColor)>,
            expected_frame_pixels: &str,
        ) {
            let color_symbols: HashMap<[u8; 4], &str> =
                HashMap::from_iter(color_symbols.iter().map(|(symbol, color)| match color {
                    BrpColor::Transparent => ([0_u8, 0_u8, 0_u8, 0_u8], *symbol),
                    BrpColor::Solid { r, g, b } => ([*r, *g, *b, 255_u8], *symbol),
                }));

            let expected_frame_pixels_lines: Vec<&str> = expected_frame_pixels
                .split('\n')
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect();

            let expected_frame_pixels = expected_frame_pixels_lines.join("\n");

            let mut actual_frame_pixels = "".to_string();
            for y in 0..self.canvas_size.y {
                actual_frame_pixels += "\n";
                for x in 0..self.canvas_size.x {
                    let pixel: [u8; PX_LEN] = self.get_pixel(ivec2(x as i32, y as i32));
                    match color_symbols.get(&pixel) {
                        Some(symbol) => actual_frame_pixels += symbol,
                        None => actual_frame_pixels += "?",
                    }
                }
            }
            let actual_frame_pixels = actual_frame_pixels.as_str().trim();

            assert_eq!(actual_frame_pixels, expected_frame_pixels);
        }

        fn get_pixel(&self, xy: IVec2) -> [u8; PX_LEN] {
            let idx = self
                .draw
                .frame_index_of(xy)
                .expect("should convert XY to pixel index");
            [
                self.frame[idx],
                self.frame[idx + 1],
                self.frame[idx + 2],
                self.frame[idx + 3],
            ]
        }
    }
}
