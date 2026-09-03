use crate::{color::Color, vec2::Vec2};

#[derive(Clone, Copy)]
pub struct SdfResult {
    pub sd: f64,           // 符号距离 (Signed Distance)
    pub reflectivity: f64, // 反射率 (0~1)
    pub eta: f64,          // 折射率 (相对值)
    pub emissive: Color,   // 自发光颜色
    pub absorption: Color, // 吸收系数 (用于 Beer–Lambert)
}

pub trait Shape: Send + Sync {
    fn sdf_result(&self, p: Vec2) -> SdfResult;
    fn sdf(&self, p: Vec2) -> f64 {
        self.sdf_result(p).sd
    }
}
