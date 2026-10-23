use crate::ambient::Ambient;
mod beer_lambert_scene;
mod reflect_scene;
mod refraction_scene;
mod shape_scene;

pub use beer_lambert_scene::beer_lambert_scene;
pub use reflect_scene::reflect_scene;
pub use refraction_scene::refraction_scene;
pub use shape_scene::shape_scene;

use crate::Shape;
use crate::config::RenderConfig;
use crate::sdf::load_scene_from_file;

pub fn build_scene(config: &RenderConfig) -> (Box<dyn Shape>, Ambient) {
    if let Some(ref path) = config.scene_file {
        load_scene_from_file(path)
    } else {
        match config.scene.as_str() {
            "shape" => (shape_scene(), Ambient::None),
            "reflect" => (
                reflect_scene(),
                Ambient::gradient([0.10, 0.12, 0.20], [0.02, 0.02, 0.04]),
            ),
            "refraction" => (
                refraction_scene(),
                Ambient::gradient([0.05, 0.08, 0.15], [0.00, 0.00, 0.00]),
            ),
            "beer" => (beer_lambert_scene(), Ambient::constant(0.04, 0.04, 0.08)),
            _ => (beer_lambert_scene(), Ambient::default()),
        }
    }
}
