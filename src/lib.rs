pub mod color;
pub mod vec2;
pub mod sdf;
pub mod render_config;

use std::time::Instant;
use chrono::{FixedOffset, Utc};
use image::ImageBuffer;
use rayon::prelude::*;
use render_config::RenderConfig;
use sdf::sample;
use vec2::Vec2;

pub fn render(config: &RenderConfig) {
    let start_time = Instant::now();
    let mut img = ImageBuffer::new(config.width, config.height);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let u = x as f64 / config.width as f64;
        let v = y as f64 / config.height as f64;
        let color = sample(Vec2::new(u, v), config.sample_count);
        let r = (color.r.min(1.0) * 255.0) as u8;
        let g = (color.g.min(1.0) * 255.0) as u8;
        let b = (color.b.min(1.0) * 255.0) as u8;
        *pixel = image::Rgb([r, g, b]);
    });

    let utc_now = Utc::now();
    let offset_eight = FixedOffset::east_opt(8 * 3600).unwrap();
    let utc_plus_eight = utc_now.with_timezone(&offset_eight);
    let timestamp = utc_plus_eight.format("%Y%m%d%H%M%S").to_string();
    let filename = format!("output/output_N{}_{}_{}.png", config.sample_count, config.output_name, timestamp);

    img.save(&filename).unwrap();
    println!("Image saved to {}", filename);
    let duration = start_time.elapsed();
    println!("Execution time: {:?}", duration);
}