use itertools::{Itertools, iproduct};
use ndarray::prelude::*;

use crate::{
    linear_algebra::{cross_product, revolve_point, vector_average, vector_normalized, vector_sub},
    polygon::{Polygon, Vertex},
};

/// ポーンの輪郭 (右半分)
pub const PAWN_CONTOUR: [[f64; 2]; 28] = [
    [0.0, 1.0],
    [0.06, 0.985],
    [0.1, 0.96],
    [0.125, 0.935],
    [0.165, 0.87],
    [0.16, 0.81],
    [0.14, 0.765],
    [0.11, 0.735],
    [0.09, 0.715],
    [0.1, 0.71],
    [0.125, 0.69],
    [0.17, 0.69],
    [0.18, 0.66],
    [0.16, 0.66],
    [0.115, 0.635],
    [0.11, 0.615],
    [0.095, 0.48],
    [0.125, 0.265],
    [0.15, 0.175],
    [0.19, 0.11],
    [0.21, 0.1],
    [0.18, 0.085],
    [0.225, 0.075],
    [0.235, 0.05],
    [0.225, 0.03],
    [0.25, 0.02],
    [0.225, 0.0],
    [0.0, 0.0],
];

pub fn create_pawn_mesh() -> impl Iterator<Item = Polygon> {
    const NUM_DIVISIONS: usize = 8;

    // let create_normal_for_quadrangle = |vertices: [Vertex; 4]| {
    //     let [v1, v2, v3, _] = vertices;
    //     let v21 = vector_truncated(vector_sub(v1.coord, v2.coord));
    //     let v23 = vector_truncated(vector_sub(v3.coord, v2.coord));
    //     vector_normalized(cross_product(v21, v23))
    // };

    // 四角形を構成する2つのポリゴンを生成するクロージャ
    let create_polygons_for_quadrangle =
        |vertices: [Vertex; 4], surround_normals: [[[f64; 3]; 3]; 3]| {
            let normals: [[f64; 3]; 4] = std::array::from_fn(|offset| {
                let (offset_row, offset_col) = (offset / 2, offset % 2);
                let normals = iproduct!(0..2, 0..2)
                    .map(|(dr, dc)| surround_normals[offset_row + dr][offset_col + dc]);
                vector_average(normals)
            });

            let vertices = std::array::from_fn(|i| {
                let mut vertex = vertices[i];
                vertex.info.normal = normals[i];
                vertex
            });

            let [v1, v2, v3, v4] = vertices;
            let vertices1 = [v1, v2, v3];
            let vertices2 = [v2, v3, v4];
            [vertices1, vertices2].map(|vertices| Polygon { vertices })
        };

    let azimuths: [f64; NUM_DIVISIONS] =
        std::array::from_fn(|k| std::f64::consts::TAU * k as f64 / NUM_DIVISIONS as f64);

    let normal_array = PAWN_CONTOUR
        .into_iter()
        .tuple_windows()
        .map(|(point1, point2)| {
            azimuths
                .into_iter()
                .circular_tuple_windows()
                .map(move |(azimuth1, azimuth2)| {
                    let coord1 = revolve_point(point1, azimuth1);
                    let coord2 = revolve_point(point1, azimuth2);
                    let coord3 = revolve_point(point2, azimuth1);
                    let c12 = vector_sub(coord2, coord1);
                    let c13 = vector_sub(coord3, coord1);
                    vector_normalized(cross_product(c13, c12))
                })
                .collect_vec()
        })
        .collect_vec();

    let get_surround_normals = move |i: usize, j: usize| {
        [-1, 0, 1].map(|dr| {
            [-1, 0, 1].map(|dc| {
                let row = (i as isize + dr).clamp(0, PAWN_CONTOUR.len() as isize - 2) as usize;
                let col = (j as isize + dc).clamp(0, NUM_DIVISIONS as isize) as usize;
                normal_array[row][col % NUM_DIVISIONS]
            })
        })
    };

    let mut upper_left_polygon_array =
        Array2::<Polygon>::default((PAWN_CONTOUR.len() - 1, NUM_DIVISIONS));
    let mut lower_right_polygon_array =
        Array2::<Polygon>::default((PAWN_CONTOUR.len() - 1, NUM_DIVISIONS));
    for ((i, (point1, point2)), (j, (azimuth1, azimuth2))) in iproduct!(
        PAWN_CONTOUR.into_iter().tuple_windows().enumerate(),
        azimuths.into_iter().circular_tuple_windows().enumerate()
    ) {
        let vertices = [
            Vertex::from_revolving(point1, azimuth1),
            Vertex::from_revolving(point1, azimuth2),
            Vertex::from_revolving(point2, azimuth1),
            Vertex::from_revolving(point2, azimuth2),
        ];
        let surround_normals = get_surround_normals(i, j);
        let [upper_left_polygon, lower_right_polygon] =
            create_polygons_for_quadrangle(vertices, surround_normals);
        upper_left_polygon_array[(i, j)] = upper_left_polygon;
        lower_right_polygon_array[(i, j)] = lower_right_polygon;
    }

    upper_left_polygon_array
        .into_iter()
        .interleave(lower_right_polygon_array)
}
