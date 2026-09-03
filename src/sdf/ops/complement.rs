use crate::{
    sdf::core::{SdfResult, Shape},
    vec2::Vec2,
};

pub struct Complement {
    a: Box<dyn Shape>,
}

impl Complement {
    pub fn new(a: Box<dyn Shape>) -> Self {
        Self { a }
    }
}

impl Shape for Complement {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        let ra = self.a.sdf_result(p);
        let mut result = ra;
        result.sd = -result.sd;
        result
    }
}
