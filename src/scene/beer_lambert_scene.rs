use crate::{
    Shape,
    color::Color,
    sdf::{Circle, Ngon, Union},
    vec2::Vec2,
};

pub fn beer_lambert_scene() -> Box<dyn Shape> {
    let light = Circle::new(Vec2::new(0.5, -0.2), 0.1).with_emissive(Color::new(10.0, 10.0, 10.0));
    let glass = Ngon::new(Vec2::new(0.5, 0.5), 0.25, 5.0)
        .with_eta(1.5)
        .with_absorption(Color::new(1.41, 1.73, 1.14));

    Box::new(Union::new(Box::new(light), Box::new(glass)))
}
