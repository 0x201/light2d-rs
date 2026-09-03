use std::ops::{Add, Mul};

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}
