use crate::{
    sdf::{SdfResult, Shape},
    vec2::Vec2,
};

pub struct Offset {
    shape: Box<dyn Shape>,
    distance: f64,
}

impl Offset {
    pub fn new(shape: Box<dyn Shape>, distance: f64) -> Self {
        Self { shape, distance }
    }
}

impl Shape for Offset {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        let mut r = self.shape.sdf_result(p);
        r.sd += self.distance;
        r
    }
}
