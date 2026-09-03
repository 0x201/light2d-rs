use crate::color::WHITE;
use crate::sdf::{Offset, Shape, Triangle, Union};
use crate::vec2::Vec2;

pub fn shape_scene() -> Box<dyn Shape> {
    let f = Triangle::new(
        Vec2::new(0.5, 0.2),
        Vec2::new(0.8, 0.8),
        Vec2::new(0.3, 0.6),
    )
    .with_emissive(WHITE);

    let g = Offset::new(
        Box::new(
            Triangle::new(
                Vec2::new(0.5, 0.2),
                Vec2::new(0.8, 0.8),
                Vec2::new(0.3, 0.6),
            )
            .with_emissive(WHITE),
        ),
        -0.1,
    );

    Box::new(Union::new(Box::new(f), Box::new(g)))
}
