use crate::utils::gamma_correction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RgbU8(pub [u8; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RgbF64(pub [f64; 3]);

impl RgbF64 {
    pub fn to_u8(self) -> RgbU8 {
        let convert = |value: f64| (gamma_correction(value) * 256.0).clamp(0.0, 255.0) as u8;
        RgbU8(self.0.map(convert))
    }
}
