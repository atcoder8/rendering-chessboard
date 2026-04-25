use itertools::Itertools;

use crate::polygon::{Polygon, Vertex};

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

    // 四角形を構成する2つのポリゴンを生成するクロージャ
    let create_polygons_for_quadrangle = |vertices: [Vertex; 4]| {
        let [v1, v2, v3, v4] = vertices;
        [
            Polygon {
                vertices: [v1, v2, v3],
            },
            Polygon {
                vertices: [v1, v3, v4],
            },
        ]
    };

    let azimuths =
        (0..NUM_DIVISIONS).map(|k| std::f64::consts::TAU * k as f64 / NUM_DIVISIONS as f64);

    PAWN_CONTOUR
        .into_iter()
        .tuple_windows()
        .flat_map(move |(point1, point2)| {
            azimuths
                .clone()
                .circular_tuple_windows()
                .flat_map(move |(azimuth1, azimuth2)| {
                    let vertices = [
                        Vertex::from_2d_coord(point1, azimuth1),
                        Vertex::from_2d_coord(point1, azimuth2),
                        Vertex::from_2d_coord(point2, azimuth1),
                        Vertex::from_2d_coord(point2, azimuth2),
                    ];
                    create_polygons_for_quadrangle(vertices)
                })
        })
}
