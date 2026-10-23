use crate::color::{BLACK, Color};
use crate::vec2::Vec2;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Ambient {
    None,
    Constant {
        #[serde(default)]
        color: [f64; 3],
    },
    Gradient {
        #[serde(default = "default_sky")]
        top: [f64; 3],
        #[serde(default = "default_ground")]
        bottom: [f64; 3],
    },
}

fn default_sky() -> [f64; 3] {
    [0.10, 0.15, 0.25]
}
fn default_ground() -> [f64; 3] {
    [0.02, 0.02, 0.02]
}

impl Default for Ambient {
    fn default() -> Self {
        Ambient::None
    }
}

impl Ambient {
    pub fn sample(&self, direction: Vec2) -> Color {
        match self {
            Ambient::None => BLACK,
            Ambient::Constant { color } => Color::new(color[0], color[1], color[2]),
            Ambient::Gradient { top, bottom } => {
                // direction.y ∈ [-1, 1] → t ∈ [0, 1]
                let t = (direction.y * 0.5 + 0.5).clamp(0.0, 1.0);
                Color::new(
                    bottom[0] + (top[0] - bottom[0]) * t,
                    bottom[1] + (top[1] - bottom[1]) * t,
                    bottom[2] + (top[2] - bottom[2]) * t,
                )
            }
        }
    }

    pub fn constant(r: f64, g: f64, b: f64) -> Self {
        Ambient::Constant { color: [r, g, b] }
    }

    pub fn gradient(top: [f64; 3], bottom: [f64; 3]) -> Self {
        Ambient::Gradient { top, bottom }
    }
}
