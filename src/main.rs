use light2d_rs::{config::RenderConfig, render::render, scene::build_scene};

fn main() {
    let config = RenderConfig::from_args();
    let (scene, ambient) = build_scene(&config);
    render(&config, &*scene, &ambient);
}
