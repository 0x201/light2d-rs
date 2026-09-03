use crate::{
    Shape,
    color::{BLACK, Color},
    sdf::{SdfResult, capsule_sdf},
    vec2::Vec2,
};

pub struct Capsule {
    a: Vec2,
    b: Vec2,
    radius: f64,
    reflectivity: f64,
    eta: f64,
    emissive: Color,
    absorption: Color,
}

impl Capsule {
    pub fn new(a: Vec2, b: Vec2, radius: f64) -> Self {
        Self {
            a,
            b,
            radius,
            reflectivity: 0.0,
            eta: 0.0,
            emissive: BLACK,
            absorption: BLACK,
        }
    }

    pub fn with_reflectivity(mut self, r: f64) -> Self {
        self.reflectivity = r;
        self
    }
    pub fn with_eta(mut self, eta: f64) -> Self {
        self.eta = eta;
        self
    }
    pub fn with_emissive(mut self, c: Color) -> Self {
        self.emissive = c;
        self
    }
    pub fn with_absorption(mut self, c: Color) -> Self {
        self.absorption = c;
        self
    }
}

impl Shape for Capsule {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        SdfResult {
            sd: capsule_sdf(p, self.a, self.b, self.radius),
            reflectivity: self.reflectivity,
            eta: self.eta,
            emissive: self.emissive,
            absorption: self.absorption,
        }
    }
}
