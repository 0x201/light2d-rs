use light2d_rs::render_config::RenderConfig;
use light2d_rs::render;

fn main() {
    let config = RenderConfig::from_args();

    render(&config);
}
