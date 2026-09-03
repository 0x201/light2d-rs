use crate::{
    Shape,
    color::{BLACK, Color},
    sdf::{SdfResult, plane_sdf},
    vec2::Vec2,
};

pub struct Plane {
    point: Vec2,
    normal: Vec2,
    reflectivity: f64,
    eta: f64,
    emissive: Color,
    absorption: Color,
}

impl Plane {
    pub fn new(point: Vec2, normal: Vec2) -> Self {
        let len = normal.length();
        let normal = if len > 0.0 {
            normal * (1.0 / len)
        } else {
            Vec2::new(0.0, 1.0)
        };
        Self {
            point,
            normal,
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

impl Shape for Plane {
    fn sdf_result(&self, p: Vec2) -> SdfResult {
        SdfResult {
            sd: plane_sdf(self.point, p, self.normal),
            reflectivity: self.reflectivity,
            eta: self.eta,
            emissive: self.emissive,
            absorption: self.absorption,
        }
    }
}
