use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use itertools::iproduct;
use ndarray::prelude::*;

use crate::color::{RgbF64, RgbU8};

mod color;
mod utils;

fn composite_uv(uv1: [f64; 2], uv2: [f64; 2], ratio: f64) -> [f64; 2] {
    std::array::from_fn(|axis| (1.0 - ratio) * uv1[axis] + ratio * uv2[axis])
}

#[derive(Debug, Clone)]
struct Screen {
    width: usize,
    height: usize,
    canvas: Array2<RgbU8>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            canvas: Array2::default([height, width]),
        }
    }

    fn file_size(&self) -> usize {
        self.canvas.len() * 3 + 0x36
    }

    #[allow(unused)]
    fn draw_chess_board(&mut self) {
        const LIGHT_RGB: [f64; 3] = [0.9473, 0.8713, 0.7991];
        const DARK_RGB: [f64; 3] = [0.1683, 0.0742, 0.0886];

        let square_width = self.width / 8;
        let square_height = self.height / 8;

        for (row, col) in iproduct!(0..self.height, 0..self.width) {
            let rgb = if col / square_width % 2 == row / square_height % 2 {
                LIGHT_RGB
            } else {
                DARK_RGB
            };
            self.canvas[(row, col)] = RgbF64(rgb).to_u8();
        }
    }

    fn to_canvas_x(&self, x: f64) -> isize {
        ((x + 1.0) * 0.5 * self.width as f64) as isize
    }

    fn to_canvas_y(&self, y: f64) -> isize {
        ((y + 1.0) * 0.5 * self.height as f64) as isize
    }

    // fn to_canvas_coord(&self, coord: [f64; 2]) -> [isize; 2] {
    //     let [x, y] = coord;
    //     [self.to_canvas_x(x), self.to_canvas_y(y)]
    // }

    fn to_canvas_x_with_clamp(&self, x: f64) -> usize {
        self.to_canvas_x(x).clamp(0, self.width as isize - 1) as usize
    }

    fn to_canvas_y_with_clamp(&self, y: f64) -> usize {
        self.to_canvas_y(y).clamp(0, self.height as isize - 1) as usize
    }

    // fn to_canvas_coord_with_clamp(&self, coord: [f64; 2]) -> [usize; 2] {
    //     let [x, y] = coord;
    //     [
    //         self.to_canvas_x_with_clamp(x),
    //         self.to_canvas_y_with_clamp(y),
    //     ]
    // }

    // fn draw_triangle_based_on_horizontal_line(
    //     &mut self,
    //     base_x1: f64,
    //     base_x2: f64,
    //     base_y: f64,
    //     top_x: f64,
    //     top_y: f64,
    // ) {
    //     let mut y_range = [base_y, top_y].map(|y| self.to_canvas_y_with_clamp(y));
    //     y_range.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
    //     let [min_y, max_y] = y_range;
    //     for y in min_y..=max_y {
    //         let t = (y as isize - self.to_canvas_y(base_y)).abs() as f64 / (max_y - min_y) as f64;
    //         let left = self.to_canvas_x_with_clamp((1.0 - t) * base_x1 + t * top_x);
    //         let right = self.to_canvas_x_with_clamp((1.0 - t) * base_x2 + t * top_x);
    //         for x in left..=right {
    //             self.canvas[(y as usize, x)] = RgbF64([0.9, 0.7, 0.5]).to_u8();
    //         }
    //     }
    // }

    fn draw_triangle_based_on_horizontal_line_with_uv(
        &mut self,
        base_x1: f64,
        base_x2: f64,
        base_y: f64,
        top_x: f64,
        top_y: f64,
        base_uv_1: [f64; 2],
        base_uv_2: [f64; 2],
        top_uv: [f64; 2],
    ) {
        let mut y_range = [base_y, top_y].map(|y| self.to_canvas_y_with_clamp(y));
        y_range.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
        let [min_y, max_y] = y_range;
        if min_y == max_y {
            return;
        }
        for y in min_y..=max_y {
            let ratio_v =
                (y as isize - self.to_canvas_y(top_y)).abs() as f64 / (max_y - min_y) as f64;
            let left = self.to_canvas_x_with_clamp((1.0 - ratio_v) * top_x + ratio_v * base_x1);
            let right = self.to_canvas_x_with_clamp((1.0 - ratio_v) * top_x + ratio_v * base_x2);
            let left_uv = composite_uv(top_uv, base_uv_1, ratio_v);
            let right_uv = composite_uv(top_uv, base_uv_2, ratio_v);
            for x in left..=right {
                let ratio_h = (x - left) as f64 / (right - left) as f64;
                let [u, v] = composite_uv(left_uv, right_uv, ratio_h);
                self.canvas[(y, x)] = RgbF64([u, v, 0.5]).to_u8();
            }
        }
    }

    fn draw_triangle(
        &mut self,
        a: [f64; 2],
        b: [f64; 2],
        c: [f64; 2],
        a_uv: [f64; 2],
        b_uv: [f64; 2],
        c_uv: [f64; 2],
    ) {
        let mut coords = [(a, a_uv), (b, b_uv), (c, c_uv)];
        coords.sort_unstable_by(|x, y| x.0[1].partial_cmp(&y.0[1]).unwrap());

        if self.to_canvas_y(coords[0].0[1]) == self.to_canvas_y(coords[2].0[1]) {
            return;
        }

        let [([ax, ay], a_uv), ([bx, by], b_uv), ([cx, cy], c_uv)] = coords;
        let t = (by - ay) / (cy - ay);
        let ex = (1.0 - t) * ax + t * cx;
        let e_uv = composite_uv(a_uv, c_uv, t);
        for ((top_x, top_y), top_uv) in [((ax, ay), a_uv), ((cx, cy), c_uv)] {
            self.draw_triangle_based_on_horizontal_line_with_uv(
                bx, ex, by, top_x, top_y, b_uv, e_uv, top_uv,
            );
        }
    }

    fn canvas_to_bytes(&self) -> impl IntoIterator<Item = u8> {
        self.canvas
            .iter()
            .flat_map(|pixel| pixel.0.into_iter().rev())
    }

    fn write_bin(&self) -> Vec<u8> {
        let mut bin: Vec<u8> = vec![];
        bin.reserve(self.file_size());

        bin.extend([0x42, 0x4d]);

        let uint_values = [
            self.file_size() as u32,
            0x00,
            0x36,
            0x28,
            self.width as u32,
            self.height as u32,
            0x00180001,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        bin.extend(
            uint_values
                .into_iter()
                .flat_map(|value| value.to_le_bytes()),
        );

        bin.extend(self.canvas_to_bytes());

        bin
    }

    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let f = File::create(path)?;
        let mut reader = BufWriter::new(f);
        reader.write(&self.write_bin())?;

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 1024;

    let mut screen = Screen::new(WIDTH, HEIGHT);
    // screen.draw_chess_board();
    screen.draw_triangle(
        [-0.5, 0.5],
        [-0.8, 0.2],
        [0.3, -0.4],
        [0.0, 0.0],
        [1.0, 0.0],
        [0.0, 1.0],
    );
    screen.save_to_file("output.bmp")?;

    Ok(())
}
