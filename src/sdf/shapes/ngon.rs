use crate::{
    Shape,
    color::{BLACK, Color},
    sdf::{SdfResult, ngon_sdf},
    vec2::Vec2,
};

pub struct Ngon {
    center: Vec2,
    radius: f64,
    n: f64,
    reflectivity: f64,
    eta: f64,
    emissive: Color,
    absorption: Color,
}

impl Ngon {
    pub fn new(center: Vec2, radius: f64, n: f64) -> Self {
        Self {
            center,
            radius,
            n,
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

impl Shape for Ngon {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        SdfResult {
            sd: ngon_sdf(p, self.center, self.radius, self.n),
            reflectivity: self.reflectivity,
            eta: self.eta,
            emissive: self.emissive,
            absorption: self.absorption,
        }
    }
}
