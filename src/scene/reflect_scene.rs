use crate::color::Color;
use crate::sdf::{BoxShape, TWO_PI};
use crate::sdf::{Circle, Shape, Union};
use crate::vec2::Vec2;

pub fn reflect_scene() -> Box<dyn Shape> {
    let a = Circle::new(Vec2::new(0.4, 0.2), 0.1).with_emissive(Color::new(2.0, 2.0, 2.0));
    let b = BoxShape::new(Vec2::new(0.5, 0.8), TWO_PI / 16.0, Vec2::new(0.1, 0.1))
        .with_reflectivity(0.9);
    let c = BoxShape::new(Vec2::new(0.8, 0.5), TWO_PI / 16.0, Vec2::new(0.1, 0.1))
        .with_reflectivity(0.9);

    Box::new(Union::new(
        Box::new(a),
        Box::new(Union::new(Box::new(b), Box::new(c))),
    ))
}
