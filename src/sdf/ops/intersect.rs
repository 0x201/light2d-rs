use crate::{Shape, sdf::SdfResult, vec2::Vec2};

pub struct Intersect {
    a: Box<dyn Shape>,
    b: Box<dyn Shape>,
}

impl Intersect {
    pub fn new(a: Box<dyn Shape>, b: Box<dyn Shape>) -> Self {
        Self { a, b }
    }
}

impl Shape for Intersect {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        let ra = self.a.sdf_result(p);
        let rb = self.b.sdf_result(p);
        let sd = ra.sd.max(rb.sd);
        SdfResult { sd, ..ra }
    }
}
