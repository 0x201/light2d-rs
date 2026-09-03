use crate::color::Color;
use crate::sdf::{Circle, Intersect, Plane, Shape, Union};
use crate::vec2::Vec2;

pub fn refraction_scene() -> Box<dyn Shape> {
    let light = Circle::new(Vec2::new(0.5, -0.5), 0.05).with_emissive(Color::new(20.0, 20.0, 20.0));
    let circle = Circle::new(Vec2::new(0.5, 0.5), 0.2)
        .with_reflectivity(0.2)
        .with_eta(1.5);
    let plane = Plane::new(Vec2::new(0.5, 0.5), Vec2::new(0.0, -1.0))
        .with_reflectivity(0.2)
        .with_eta(1.5);
    let intersect = Intersect::new(Box::new(circle), Box::new(plane));

    Box::new(Union::new(Box::new(light), Box::new(intersect)))
}
