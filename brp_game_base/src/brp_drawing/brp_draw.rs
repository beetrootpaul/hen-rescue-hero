use bevy::math::{ivec2, IVec2, UVec2};
use bevy::utils::HashMap;

use rect;
use {BrpColor, Rect};

// amount of bytes per pixel in `pixels` crate's frame buffer
const PX_LEN: usize = 4;

pub struct BrpDraw {
    pub canvas_size: UVec2,
    pub clipping_rect: Option<Rect>,
}

impl BrpDraw {
    pub fn clear(&self, frame: &mut [u8], color: BrpColor) {
        if let BrpColor::Solid { r, g, b } = color {
            frame.copy_from_slice(&[r, g, b, 0xff].repeat(frame.len() / PX_LEN));
        }
    }

    // TODO: make use of self.clipping_rect
    pub fn draw_pixel(&self, frame: &mut [u8], xy: IVec2, color: BrpColor) {
        self.set_pixel(frame, xy, color);
    }

    // TODO: make use of self.clipping_rect
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

    // TODO: make use of self.clipping_rect
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

    pub fn draw_sprite(
        &self,
        frame: &mut [u8],
        xy: IVec2,
        image_w: usize,
        source_image_data: &[u8],
        source_rect: Rect,
        color_replacements: HashMap<BrpColor, BrpColor>,
    ) {
        let target_rect = source_rect.at(xy.x, xy.y);
        let clipped_target_rect =
            target_rect.intersection_with(rect(self.canvas_size.x, self.canvas_size.y));
        let clipped_target_rect = match self.clipping_rect {
            Some(clipping_rect) => clipped_target_rect.intersection_with(clipping_rect),
            None => clipped_target_rect,
        };

        let left_top_diff = clipped_target_rect.left_top - target_rect.left_top;
        let size_diff = clipped_target_rect.size.as_ivec2() - target_rect.size.as_ivec2();

        let clipped_source_rect = Rect {
            left_top: source_rect.left_top + left_top_diff,
            size: (source_rect.size.as_ivec2() + size_diff).as_uvec2(),
        };

        if let Some(pixel_index) = self.frame_index_of(clipped_target_rect.left_top) {
            let sprite_w = clipped_source_rect.width() as usize;
            let sprite_h = clipped_source_rect.height() as usize;

            for sprite_row in 0..sprite_h {
                for sprite_column in 0..sprite_w {
                    let target_i = pixel_index
                        + (sprite_row * (self.canvas_size.x as usize) + sprite_column) * PX_LEN;
                    let source_i = ((clipped_source_rect.top() as usize + sprite_row) * image_w
                        + (clipped_source_rect.left() as usize + sprite_column))
                        * PX_LEN;
                    let source_rgba = &source_image_data[source_i..(source_i + PX_LEN)];

                    match color_replacements.get(&BrpColor::Solid {
                        r: source_rgba[0],
                        g: source_rgba[1],
                        b: source_rgba[2],
                    }) {
                        Some(replacement_color) => {
                            if let BrpColor::Solid { r, g, b } = replacement_color {
                                frame[target_i] = *r;
                                frame[target_i + 1] = *g;
                                frame[target_i + 2] = *b;
                                frame[target_i + 3] = 0xff;
                            }
                        },
                        _ => {
                            frame[target_i] = source_rgba[0];
                            frame[target_i + 1] = source_rgba[1];
                            frame[target_i + 2] = source_rgba[2];
                            frame[target_i + 3] = source_rgba[3];
                        },
                    };
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

    #[test]
    fn test_draw_sprite() {
        let mut h = TestHelper::for_canvas_size(6, 4);
        let color_bg = color_solid(9, 8, 7);
        let color_1 = color_solid(11, 12, 13);
        let color_2 = color_solid(21, 22, 23);
        let color_3 = color_solid(31, 32, 33);
        let color_symbols = vec![
            ("-", color_bg),
            (":", color_1),
            ("#", color_2),
            ("%", color_3),
        ];
        let img = TestImage::from_pixels(
            color_symbols.clone(),
            "
            :%%%%%:
            %::%::%
            %#%:%#%
            %::%::%
            :%%%%%:
        ",
        );

        h.draw.clear(&mut h.frame, color_bg);
        h.draw.draw_sprite(
            &mut h.frame,
            ivec2(1, 1),
            img.width,
            &img.data,
            rect(4, 2).at(1, 2),
            HashMap::new(),
        );

        h.assert_frame_pixels(
            color_symbols,
            "
                ------
                -#%:%-
                -::%:-
                ------
            ",
        );
    }

    #[test]
    fn test_draw_sprites_clipped() {
        let mut h = TestHelper::for_canvas_size(5, 5);
        let color_bg = color_solid(9, 8, 7);
        let color_1 = color_solid(11, 12, 13);
        let color_2 = color_solid(21, 22, 23);
        let color_3 = color_solid(31, 32, 33);
        let color_4 = color_solid(41, 42, 43);
        let color_5 = color_solid(51, 52, 53);
        let color_symbols = vec![
            ("-", color_bg),
            (":", color_1),
            ("#", color_2),
            ("%", color_3),
            ("$", color_4),
            ("!", color_5),
        ];
        let img1 = TestImage::from_pixels(
            color_symbols.clone(),
            "
            :::
            :::
            :::
        ",
        );
        let img2 = TestImage::from_pixels(
            color_symbols.clone(),
            "
            ###
            ###
            ###
        ",
        );
        let img3 = TestImage::from_pixels(
            color_symbols.clone(),
            "
            %%%
            %%%
            %%%
        ",
        );
        let img4 = TestImage::from_pixels(
            color_symbols.clone(),
            "
            $$$
            $$$
            $$$
        ",
        );
        let img5 = TestImage::from_pixels(
            color_symbols.clone(),
            "
            !!!
            !!!
            !!!
        ",
        );

        h.draw.clear(&mut h.frame, color_bg);
        // clipped from the left
        h.draw.draw_sprite(
            &mut h.frame,
            ivec2(-1, 1),
            img1.width,
            &img1.data,
            rect(3, 3),
            HashMap::new(),
        );
        // clipped from the right
        h.draw.draw_sprite(
            &mut h.frame,
            ivec2(3, 1),
            img2.width,
            &img2.data,
            rect(3, 3),
            HashMap::new(),
        );
        // clipped from the top
        h.draw.draw_sprite(
            &mut h.frame,
            ivec2(1, -1),
            img3.width,
            &img3.data,
            rect(3, 3),
            HashMap::new(),
        );
        // clipped from the bottom
        h.draw.draw_sprite(
            &mut h.frame,
            ivec2(1, 3),
            img4.width,
            &img4.data,
            rect(3, 3),
            HashMap::new(),
        );
        // drawn last, but clipped entirely
        h.draw.draw_sprite(
            &mut h.frame,
            ivec2(-3, 0),
            img5.width,
            &img5.data,
            rect(3, 3),
            HashMap::new(),
        );

        h.assert_frame_pixels(
            color_symbols,
            "
                -%%%-
                :%%%#
                ::-##
                :$$$#
                -$$$-
            ",
        );
    }
    #[test]
    fn test_draw_sprite_with_color_replacements() {
        let mut h = TestHelper::for_canvas_size(4, 2);
        let color_bg = color_solid(9, 8, 7);
        let color_1 = color_solid(11, 12, 13);
        let color_2 = color_solid(21, 22, 23);
        let color_3 = color_solid(31, 32, 33);
        let color_4 = color_solid(41, 42, 43);
        let color_5 = color_solid(51, 52, 53);
        let color_symbols = vec![
            ("-", color_bg),
            (":", color_1),
            ("#", color_2),
            ("%", color_3),
            ("$", color_4),
            ("!", color_5),
        ];
        let img = TestImage::from_pixels(
            color_symbols.clone(),
            "
            %:%:
            :#:#
        ",
        );

        h.draw.clear(&mut h.frame, color_bg);
        h.draw.draw_sprite(
            &mut h.frame,
            IVec2::ZERO,
            img.width,
            &img.data,
            rect(4, 2),
            HashMap::from([(color_1, BrpColor::Transparent), (color_2, color_4)]),
        );

        h.assert_frame_pixels(
            color_symbols.clone(),
            "
                %-%-
                -$-$
            ",
        );

        h.draw.draw_sprite(
            &mut h.frame,
            IVec2::ZERO,
            img.width,
            &img.data,
            rect(4, 2),
            HashMap::from([(color_1, color_5), (color_2, BrpColor::Transparent)]),
        );

        h.assert_frame_pixels(
            color_symbols.clone(),
            "
                %!%!
                !$!$
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
                draw: BrpDraw {
                    canvas_size,
                    // TODO: test it
                    clipping_rect: None,
                },
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

            let expected_frame_pixels =
                normalized_ascii_pixel_rows(expected_frame_pixels).join("\n");

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

            println!("=========\n");
            println!("EXPECTED:\n{}\n", expected_frame_pixels);
            println!("ACTUAL:\n{}\n", actual_frame_pixels);
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

    struct TestImage {
        width: usize,
        data: Vec<u8>,
    }

    impl TestImage {
        fn from_pixels(color_symbols: Vec<(&str, BrpColor)>, image_pixels: &str) -> Self {
            let color_symbols: HashMap<char, [u8; 4]> =
                HashMap::from_iter(color_symbols.iter().map(|(symbol, color)| match color {
                    BrpColor::Transparent => {
                        (symbol.chars().last().unwrap(), [0_u8, 0_u8, 0_u8, 0_u8])
                    },
                    BrpColor::Solid { r, g, b } => {
                        (symbol.chars().last().unwrap(), [*r, *g, *b, 255_u8])
                    },
                }));

            let symbol_lines = normalized_ascii_pixel_rows(image_pixels);
            let line_width = symbol_lines
                .first()
                .expect("should have at least 1 line")
                .len();
            assert!(symbol_lines.iter().all(|line| line.len() == line_width));

            let symbols: Vec<char> = symbol_lines.iter().flat_map(|line| line.chars()).collect();
            let bytes: Vec<u8> = symbols
                .iter()
                .flat_map(|symbol| {
                    let bytes = color_symbols.get(symbol).unwrap_or_else(|| {
                        panic!(
                            "should use symbols of defined colors, but used '{}' instead",
                            symbol
                        )
                    });
                    *bytes
                })
                .collect();

            Self {
                width: line_width,
                data: bytes,
            }
        }
    }

    fn normalized_ascii_pixel_rows(ascii_pixels: &str) -> Vec<&str> {
        ascii_pixels
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect()
    }
}
