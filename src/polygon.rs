#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub coord: [f64; 4],
}

impl Vertex {
    pub fn from_2d_coord(coord: [f64; 2], azimuth: f64) -> Self {
        let [x, y] = coord;
        let (sin, cos) = azimuth.sin_cos();
        Vertex {
            coord: [x * cos, y, x * sin, 1.0],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Polygon {
    pub vertices: [Vertex; 3],
}
