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

pub fn build_scene(config: &RenderConfig) -> Box<dyn Shape> {
    if let Some(ref path) = config.scene_file {
        load_scene_from_file(path)
    } else {
        match config.scene.as_str() {
            "shape" => shape_scene(),
            "reflect" => reflect_scene(),
            "refraction" => refraction_scene(),
            "beer" => beer_lambert_scene(),
            _ => beer_lambert_scene(),
        }
    }
}
