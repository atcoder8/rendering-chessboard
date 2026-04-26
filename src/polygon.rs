use crate::utils::vector_composited;

#[derive(Debug, Clone, Copy, Default)]
pub struct VertexInfo {
    pub normal: [f64; 3],
}

impl VertexInfo {
    fn composited(self, other: Self, ratio: f64) -> Self {
        Self {
            normal: vector_composited(self.normal, other.normal, ratio),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub coord: [f64; 4],
    pub info: VertexInfo,
}

impl Vertex {
    #[allow(unused)]
    pub fn x(self) -> f64 {
        self.coord[0]
    }

    #[allow(unused)]
    pub fn y(self) -> f64 {
        self.coord[1]
    }

    #[allow(unused)]
    pub fn z(self) -> f64 {
        self.coord[2]
    }

    pub fn from_2d_coord(coord: [f64; 2], azimuth: f64) -> Self {
        let [x, y] = coord;
        let (sin, cos) = azimuth.sin_cos();
        Vertex {
            coord: [x * cos, y, x * sin, 1.0],
            info: VertexInfo::default(),
        }
    }

    pub fn composited(self, other: Self, ratio: f64) -> Self {
        Self {
            coord: vector_composited(self.coord, other.coord, ratio),
            info: self.info.composited(other.info, ratio),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Polygon {
    pub vertices: [Vertex; 3],
}
