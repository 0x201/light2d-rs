use std::time::Instant;

use chrono::{FixedOffset, Utc};
use image::{ImageBuffer, Rgb};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    RenderConfig, Shape,
    color::{BLACK, Color},
    sdf::TWO_PI,
    vec2::Vec2,
};

const BIAS: f64 = 1e-4;
const EPSILON: f64 = 1e-6;
const MAX_STEP: i32 = 32;
const MAX_DISTANCE: f64 = 5.0;
const MAX_DEPTH: i32 = 3;

fn reflect(i: Vec2, n: Vec2) -> Vec2 {
    i - n * 2.0 * i.dot(&n)
}

fn refract(i: Vec2, n: Vec2, eta: f64) -> Option<Vec2> {
    let i_dot_n = i.dot(&n);
    let k = 1.0 - eta.powi(2) * (1.0 - i_dot_n.powi(2));

    if k < 0.0 {
        // Total internal reflection
        None
    } else {
        let a = eta * i_dot_n + k.sqrt();
        Some(i * eta - n * a)
    }
}

fn fresnel(cos_i: f64, cos_t: f64, eta_i: f64, eta_t: f64) -> f64 {
    let rs = (eta_t * cos_i - eta_i * cos_t) / (eta_t * cos_i + eta_i * cos_t);
    let rp = (eta_i * cos_i - eta_t * cos_t) / (eta_i * cos_i + eta_t * cos_t);
    (rs * rs + rp * rp) * 0.5
}

#[allow(unused)]
fn schlick(cos_i: f64, cos_t: f64, eta_i: f64, eta_t: f64) -> f64 {
    let mut r0 = (eta_i - eta_t) / (eta_i + eta_t);
    r0 *= r0;
    let a = 1.0 - (if eta_i < eta_t { cos_i } else { cos_t });
    let aa = a * a;
    r0 + (1.0 - r0) * aa * aa * a
}

fn beer_lambert(absorption: Color, distance: f64) -> Color {
    Color::new(
        (-absorption.r * distance).exp(),
        (-absorption.g * distance).exp(),
        (-absorption.b * distance).exp(),
    )
}

fn gradient(p: Vec2, scene: &dyn Shape) -> Vec2 {
    let eps = EPSILON;
    let nx = (scene.sdf_result(Vec2::new(p.x + eps, p.y)).sd
        - scene.sdf_result(Vec2::new(p.x - eps, p.y)).sd)
        * (0.5 / eps);
    let ny = (scene.sdf_result(Vec2::new(p.x, p.y + eps)).sd
        - scene.sdf_result(Vec2::new(p.x, p.y - eps)).sd)
        * (0.5 / eps);
    Vec2::new(nx, ny)
}

fn trace(origin: Vec2, direction: Vec2, depth: i32, scene: &dyn Shape) -> Color {
    let mut t = 0.0;
    let sign = if scene.sdf_result(origin).sd > 0.0 {
        1.0
    } else {
        -1.0
    };
    for _ in 0..MAX_STEP {
        if t > MAX_DISTANCE {
            break;
        }
        let center = origin + direction * t;
        let r = scene.sdf_result(center);

        if r.sd * sign < EPSILON {
            let mut sum = r.emissive;

            if depth < MAX_DEPTH && r.eta > 0.0 {
                let mut refl = r.reflectivity;
                let mut n = gradient(center, scene);
                let s = 1.0 / (n.x * n.x + n.y * n.y);
                n = n * sign * s;

                if r.eta > 0.0 {
                    let eta = if sign < 0.0 { r.eta } else { 1.0 / r.eta };

                    if let Some(refracted_dir) = refract(direction, n, eta) {
                        let cos_i = -direction.dot(&n);
                        let cost = -refracted_dir.dot(&n);
                        refl = if sign < 0.0 {
                            fresnel(cos_i, cost, r.eta, 1.0)
                        } else {
                            fresnel(cos_i, cost, 1.0, r.eta)
                        };

                        // refl = if sign < 0.0 {
                        //     schlick(cos_i, cost, r.eta, 1.0)
                        // } else {
                        //     schlick(cos_i, cost, 1.0, r.eta)
                        // };

                        refl = refl.min(1.0).max(0.0);

                        sum = sum
                            + (trace(center - n * BIAS, refracted_dir, depth + 1, scene)
                                * (1.0 - refl));
                    } else {
                        refl = 1.0; // Total internal reflection
                    }
                }

                if refl > 0.0 {
                    let reflected_dir = reflect(direction, n);
                    sum = sum + trace(center + n * BIAS, reflected_dir, depth + 1, scene) * refl;
                }
            }
            return sum * beer_lambert(r.absorption, t);
        }
        t += r.sd * sign;
    }
    BLACK
}

pub fn sample_pixel(uv: Vec2, sample_count: usize, scene: &dyn Shape) -> Color {
    let sum = (0..sample_count)
        .into_par_iter()
        .fold(
            || BLACK,
            |acc, i| {
                let mut rng = rand::rng();
                let a = TWO_PI * (i as f64 + rng.random::<f64>()) / (sample_count as f64);
                let color = trace(uv, Vec2::new(a.cos(), a.sin()), 0, scene);
                acc + color
            },
        )
        .reduce(|| BLACK, |a, b| a + b);
    sum * (1.0 / sample_count as f64)
}

pub fn render(config: &RenderConfig, scene: &dyn Shape) {
    let start = Instant::now();

    let width = config.width;
    let height = config.height;
    let sample_count = config.sample_count;

    let mut img = ImageBuffer::new(width, height);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;
        let uv = Vec2::new(u, v);
        let color = sample_pixel(uv, sample_count, scene);

        let r = (color.r.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (color.g.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (color.b.clamp(0.0, 1.0) * 255.0) as u8;
        *pixel = Rgb([r, g, b]);
    });

    let now = Utc::now();
    let offset = FixedOffset::east_opt(8 * 3600).unwrap();
    let timestamp = now
        .with_timezone(&offset)
        .format("%Y%m%d%H%M%S")
        .to_string();
    let filename = format!(
        "{}/N{}_{}_{}.png",
        config.output_dir, config.sample_count, config.output_name, timestamp
    );

    if let Some(parent) = std::path::Path::new(&filename).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    img.save(&filename).unwrap();
    println!("Image saved to {}", filename);
    println!("Execution time: {:?}", start.elapsed());
}
