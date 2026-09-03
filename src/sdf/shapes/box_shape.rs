use crate::{
    Shape,
    color::{BLACK, Color},
    sdf::{SdfResult, box_sdf},
    vec2::Vec2,
};

pub struct BoxShape {
    center: Vec2,
    theta: f64,
    half_size: Vec2,
    reflectivity: f64,
    eta: f64,
    emissive: Color,
    absorption: Color,
}

impl BoxShape {
    pub fn new(center: Vec2, theta: f64, half_size: Vec2) -> Self {
        Self {
            center,
            theta,
            half_size,
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

impl Shape for BoxShape {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        SdfResult {
            sd: box_sdf(p, self.center, self.theta, self.half_size),
            reflectivity: self.reflectivity,
            eta: self.eta,
            emissive: self.emissive,
            absorption: self.absorption,
        }
    }
}
