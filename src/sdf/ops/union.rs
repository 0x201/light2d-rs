use crate::{
    sdf::core::{SdfResult, Shape},
    vec2::Vec2,
};

pub struct Union {
    a: Box<dyn Shape>,
    b: Box<dyn Shape>,
}

impl Union {
    pub fn new(a: Box<dyn Shape>, b: Box<dyn Shape>) -> Self {
        Self { a, b }
    }
}

impl Shape for Union {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        let ra = self.a.sdf_result(p);
        let rb = self.b.sdf_result(p);
        if ra.sd < rb.sd { ra } else { rb }
    }
}
