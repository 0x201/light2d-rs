use crate::{
    sdf::core::{SdfResult, Shape},
    vec2::Vec2,
};

pub struct Subtract {
    a: Box<dyn Shape>,
    b: Box<dyn Shape>,
}

impl Subtract {
    pub fn new(a: Box<dyn Shape>, b: Box<dyn Shape>) -> Self {
        Self { a, b }
    }
}

impl Shape for Subtract {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        let ra = self.a.sdf_result(p);
        let rb = self.b.sdf_result(p);
        let sd = ra.sd.max(-rb.sd);
        SdfResult { sd, ..ra }
    }
}
