use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use itertools::iproduct;
use ndarray::prelude::*;

use crate::{
    color::{RgbF64, RgbU8},
    pawn::create_pawn_mesh,
    polygon::{Polygon, Vertex, VertexInfo},
    utils::vector_resize,
};

mod color;
mod pawn;
mod polygon;
mod utils;

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

    fn to_canvas_x_with_clamp(&self, x: f64) -> usize {
        self.to_canvas_x(x).clamp(0, self.width as isize - 1) as usize
    }

    fn to_canvas_y_with_clamp(&self, y: f64) -> usize {
        self.to_canvas_y(y).clamp(0, self.height as isize - 1) as usize
    }

    fn draw_triangle_based_on_horizontal(
        &mut self,
        base_vertex_1: Vertex,
        base_vertex_2: Vertex,
        top_vertex: Vertex,
    ) {
        assert!(
            (base_vertex_1.y() - base_vertex_2.y()).abs() < 1e-9,
            "三角形の底辺はx軸と平行である必要があります。"
        );

        let mut y_range =
            [base_vertex_1.y(), top_vertex.y()].map(|y| self.to_canvas_y_with_clamp(y));
        y_range.sort_unstable();
        let [min_y, max_y] = y_range;
        for canvas_y in min_y..=max_y {
            // 底辺からの高さの割合
            let y_ratio = (canvas_y as isize - self.to_canvas_y(base_vertex_1.y())) as f64
                / (self.to_canvas_y(top_vertex.y()) - self.to_canvas_y(base_vertex_1.y())) as f64;
            // 現在の高さの左右対応する点
            let left_vertex = base_vertex_1.composited(top_vertex, y_ratio);
            let right_vertex = base_vertex_2.composited(top_vertex, y_ratio);

            let mut x_range =
                [left_vertex.x(), right_vertex.x()].map(|x| self.to_canvas_x_with_clamp(x));
            x_range.sort_unstable();
            let [min_x, max_x] = x_range;
            for canvas_x in min_x..=max_x {
                // 片側の頂点からの幅の割合
                let x_ratio = (canvas_x as isize - self.to_canvas_x(left_vertex.x())) as f64
                    / (self.to_canvas_x(right_vertex.x()) - self.to_canvas_x(left_vertex.x()))
                        as f64;
                // スクリーン座標に対応する点
                let vertex = left_vertex.composited(right_vertex, x_ratio);
                // キャンバス上に点を配置
                self.canvas[(canvas_y, canvas_x)] = RgbF64(vertex.info.normal).to_u8();
            }
        }
    }

    fn draw_polygon(&mut self, polygon: Polygon) {
        let Polygon { mut vertices } = polygon;
        vertices.sort_unstable_by(|x, y| x.y().partial_cmp(&y.y()).unwrap());
        let [va, vb, vc] = vertices;
        if va.y() == vc.y() {
            return;
        }
        let y_ratio = (vb.y() - va.y()) / (vc.y() - va.y());
        let ve = va.composited(vc, y_ratio);
        for top_vertex in [va, vc] {
            self.draw_triangle_based_on_horizontal(vb, ve, top_vertex);
        }
    }

    fn draw_pawn(&mut self) {
        let pawn_meth = create_pawn_mesh();
        for polygon in pawn_meth {
            self.draw_polygon(polygon);
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

    // チェス盤を表示
    screen.draw_chess_board();

    // 1つの三角形に対応するポリゴンを描画
    let coords = [[-0.5, 0.5, 0.0], [-0.8, 0.2, 0.0], [0.3, -0.4, 0.0]];
    let normals = [[0.0, 0.0, 0.5], [1.0, 0.0, 0.5], [0.0, 1.0, 0.5]];
    let vertices = std::array::from_fn(|axis| Vertex {
        coord: vector_resize(coords[axis], 1.0),
        info: VertexInfo {
            normal: normals[axis],
        },
    });
    screen.draw_polygon(Polygon { vertices });

    // ポーンを描画
    screen.draw_pawn();

    // 画像ファイルに出力
    screen.save_to_file("images/output.bmp")?;

    Ok(())
}
