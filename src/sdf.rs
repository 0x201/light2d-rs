#![cfg_attr(debug_assertions, allow(unused))]

use rand::prelude::*;
use rayon::prelude::*;

use crate::color::Color;
use crate::vec2::Vec2;

const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
const BIAS: f64 = 1e-4;
const EPSILON: f64 = 1e-6;
const MAX_STEP: i32 = 32;
const MAX_DISTANCE: f64 = 5.0;
const MAX_DEPTH: i32 = 3;

const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

#[derive(Clone, Copy)]
struct SdfResult {
    sd: f64,
    reflectivity: f64,
    eta: f64,
    emissive: Color,
    absorption: Color,
}

fn circle_sdf(center: Vec2, point: Vec2, radius: f64) -> f64 {
    (point - center).length() - radius
}

fn plane_sdf(o: Vec2, p: Vec2, normal: Vec2) -> f64 {
    (o - p).dot(&normal)
}

fn ngon_sdf(o: Vec2, c: Vec2, r: f64, n: f64) -> f64 {
    let u = o - c;
    let a = TWO_PI / n;

    let t = (u.y.atan2(u.x) + TWO_PI) % a;
    let s = u.length();

    let cos_t = t.cos();
    let sin_t = t.sin();
    let cos_a_half = (a * 0.5).cos();
    let sin_a_half = (a * 0.5).sin();
    plane_sdf(
        Vec2::new(s * cos_t, s * sin_t),
        Vec2::new(r, 0.0),
        Vec2::new(cos_a_half, sin_a_half),
    )
}

fn segment_sdf(o: Vec2, a: Vec2, b: Vec2) -> f64 {
    let v = o - a;
    let u = b - a;

    let t = (v.dot(&u) / u.dot(&u)).min(1.0).clamp(0.0, 1.0);

    (v - (u * t)).length()
}

fn capsule_sdf(o: Vec2, a: Vec2, b: Vec2, r: f64) -> f64 {
    segment_sdf(o, a, b) - r
}

fn box_sdf(o: Vec2, c: Vec2, theta: f64, s: Vec2) -> f64 {
    let shift = o - c;
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();

    let r = Vec2::new(
        (shift.x * cos_theta + shift.y * sin_theta).abs(),
        (shift.y * cos_theta - shift.x * sin_theta).abs(),
    );

    let d = r - s;
    let a = Vec2::new(d.x.max(0.0), d.y.max(0.0));

    d.x.max(d.y).min(0.0) + a.length()
}

fn triangle_sdf(o: Vec2, a: Vec2, b: Vec2, c: Vec2) -> f64 {
    let d = segment_sdf(o, a, b)
        .min(segment_sdf(o, b, c))
        .min(segment_sdf(o, c, a));

    let ab = b - a;
    let bc = c - b;
    let ca = a - c;

    let ao = o - a;
    let bo = o - b;
    let co = o - c;

    if ab.cross(&ao) > 0.0 && bc.cross(&bo) > 0.0 && ca.cross(&co) > 0.0 {
        -d
    } else {
        d
    }
}

fn union(a: SdfResult, b: SdfResult) -> SdfResult {
    if a.sd < b.sd { a } else { b }
}

fn intersect(a: SdfResult, b: SdfResult) -> SdfResult {
    if a.sd > b.sd { a } else { b }
}

fn subtract(a: SdfResult, b: SdfResult) -> SdfResult {
    let mut r = a;
    r.sd = if a.sd > -b.sd { a.sd } else { -b.sd };
    r
}

fn complement(mut a: SdfResult) -> SdfResult {
    a.sd = -a.sd;
    a
}

fn shape_scene(o: Vec2) -> SdfResult {
    let a = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, 0.5), 0.2),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: WHITE,
        absorption: BLACK,
    };
    let b = SdfResult {
        sd: plane_sdf(o, Vec2::new(0.0, 0.5), Vec2::new(0.0, 1.0)),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: WHITE * 0.8,
        absorption: BLACK,
    };
    let c = SdfResult {
        sd: capsule_sdf(o, Vec2::new(0.4, 0.4), Vec2::new(0.6, 0.6), 0.1),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: WHITE,
        absorption: BLACK,
    };
    let d = SdfResult {
        sd: box_sdf(o, Vec2::new(0.5, 0.5), TWO_PI / 16.0, Vec2::new(0.3, 0.1)),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: WHITE,
        absorption: BLACK,
    };
    let e = SdfResult {
        sd: box_sdf(o, Vec2::new(0.5, 0.5), TWO_PI / 16.0, Vec2::new(0.3, 0.1)) - 0.1,
        reflectivity: 0.0,
        eta: 0.0,
        emissive: WHITE,
        absorption: BLACK,
    };
    let f = SdfResult {
        sd: triangle_sdf(
            o,
            Vec2::new(0.5, 0.2),
            Vec2::new(0.8, 0.8),
            Vec2::new(0.3, 0.6),
        ),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: WHITE,
        absorption: BLACK,
    };
    let g = SdfResult {
        sd: triangle_sdf(
            o,
            Vec2::new(0.5, 0.2),
            Vec2::new(0.8, 0.8),
            Vec2::new(0.3, 0.6),
        ) - 0.1,
        reflectivity: 0.0,
        eta: 0.0,
        emissive: WHITE,
        absorption: BLACK,
    };
    // a
    // b
    // c
    // d
    // e
    // f
    // g
    //intersect(a, b)
    union(f, g)
}

fn reflect_scene(o: Vec2) -> SdfResult {
    let a = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.4, 0.2), 0.1),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: Color::new(2.0, 2.0, 2.0),
        absorption: BLACK,
    };

    let b = SdfResult {
        sd: box_sdf(o, Vec2::new(0.5, 0.8), TWO_PI / 16.0, Vec2::new(0.1, 0.1)),
        reflectivity: 0.9,
        eta: 0.0,
        emissive: BLACK,
        absorption: BLACK,
    };

    let c = SdfResult {
        sd: box_sdf(o, Vec2::new(0.8, 0.5), TWO_PI / 16.0, Vec2::new(0.1, 0.1)),
        reflectivity: 0.9,
        eta: 0.0,
        emissive: BLACK,
        absorption: BLACK,
    };

    let d = SdfResult {
        sd: plane_sdf(o, Vec2::new(0.0, 0.5), Vec2::new(0.0, -1.0)),
        reflectivity: 0.9,
        eta: 0.0,
        emissive: BLACK,
        absorption: BLACK,
    };
    let e = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, 0.5), 0.4),
        reflectivity: 0.9,
        eta: 0.0,
        emissive: BLACK,
        absorption: BLACK,
    };

    //union(a, subtract(b, c))
    union(a, union(b, c))
    //union(a, subtract(d, e))
}

fn refraction_scene(o: Vec2) -> SdfResult {
    let a = SdfResult {
        sd: circle_sdf(o, Vec2::new(-0.2, -0.2), 0.1),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: Color::new(10.0, 10.0, 10.0),
        absorption: BLACK,
    };
    let b = SdfResult {
        sd: box_sdf(o, Vec2::new(0.5, 0.5), 0.0, Vec2::new(0.3, 0.2)),
        emissive: BLACK,
        reflectivity: 0.2,
        eta: 1.5,
        absorption: BLACK,
    };
    let c = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, -0.5), 0.05),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: Color::new(20.0, 20.0, 20.0),
        absorption: BLACK,
    };
    let d = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, 0.2), 0.35),
        reflectivity: 0.2,
        eta: 1.5,
        emissive: BLACK,
        absorption: BLACK,
    };
    let e = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, 0.8), 0.35),
        reflectivity: 0.2,
        eta: 1.5,
        emissive: BLACK,
        absorption: BLACK,
    };
    let f = SdfResult {
        sd: box_sdf(o, Vec2::new(0.5, 0.5), 0.0, Vec2::new(0.2, 0.1)),
        reflectivity: 0.2,
        eta: 1.5,
        emissive: BLACK,
        absorption: BLACK,
    };
    let g = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, 0.12), 0.35),
        reflectivity: 0.2,
        eta: 1.5,
        emissive: BLACK,
        absorption: BLACK,
    };
    let h = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, 0.87), 0.35),
        reflectivity: 0.2,
        eta: 1.5,
        emissive: BLACK,
        absorption: BLACK,
    };
    let i = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, 0.5), 0.2),
        reflectivity: 0.2,
        eta: 1.5,
        emissive: BLACK,
        absorption: BLACK,
    };
    let j = SdfResult {
        sd: plane_sdf(o, Vec2::new(0.5, 0.5), Vec2::new(0.0, -1.0)),
        reflectivity: 0.2,
        eta: 1.5,
        emissive: BLACK,
        absorption: BLACK,
    };

    //union(a, b)
    //union(c, intersect(d, e))
    //union(c, subtract(f, union(g, h)))
    union(c, intersect(i, j))
}

fn beer_lambert_scene(o: Vec2) -> SdfResult {
    let a = SdfResult {
        sd: circle_sdf(o, Vec2::new(0.5, -0.2), 0.1),
        reflectivity: 0.0,
        eta: 0.0,
        emissive: Color::new(10.0, 10.0, 10.0),
        absorption: BLACK,
    };
    let b = SdfResult {
        sd: ngon_sdf(o, Vec2::new(0.5, 0.5), 0.25, 5.0),
        reflectivity: 0.0,
        eta: 1.5,
        emissive: BLACK,
        absorption: Color::new(141.0, 173.0, 114.0),
    };
    union(a, b)
}

fn gradient(p: Vec2) -> Vec2 {
    let nx = (scene(Vec2::new(p.x + EPSILON, p.y)).sd - scene(Vec2::new(p.x - EPSILON, p.y)).sd)
        * (0.5 / EPSILON);
    let ny = (scene(Vec2::new(p.x, p.y + EPSILON)).sd - scene(Vec2::new(p.x, p.y - EPSILON)).sd)
        * (0.5 / EPSILON);
    Vec2::new(nx, ny)
}

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

fn schlick(cos_i: f64, cos_t: f64, eta_i: f64, eta_t: f64) -> f64 {
    let mut r0 = (eta_i - eta_t) / (eta_i + eta_t);
    r0 *= r0;
    let a = 1.0 - (if eta_i < eta_t { cos_i } else { cos_t });
    let aa = a * a;
    r0 + (1.0 - r0) * aa * aa * a
}

fn beer_lambert(a: Color, d: f64) -> Color {
    Color::new((-a.r * d).exp(), (-a.g * d).exp(), (-a.b * d).exp())
}

fn scene(o: Vec2) -> SdfResult {
    //shape_scene(o)
    //reflect_scene(o)
    //refraction_scene(o)
    beer_lambert_scene(o)
}

fn trace(o: Vec2, d: Vec2, depth: i32) -> Color {
    let mut t = 0.0;
    let sign = if scene(o).sd > 0.0 { 1.0 } else { -1.0 };
    for _ in 0..MAX_STEP {
        if t > MAX_DISTANCE {
            break;
        }
        let center = o + d * t;
        let r = scene(center);
        if r.sd * sign < EPSILON {
            let mut sum = r.emissive;
            if depth < MAX_DEPTH && r.eta > 0.0 {
                let mut refl = r.reflectivity;
                let mut n = gradient(center);
                let s = 1.0 / (n.x * n.x + n.y * n.y);
                n = n * sign * s;

                if r.eta > 0.0 {
                    let eta = if sign < 0.0 { r.eta } else { 1.0 / r.eta };

                    if let Some(refracted_dir) = refract(d, n, eta) {
                        let cos_i = -d.dot(&n);
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
                            + (trace(center - n * BIAS, refracted_dir, depth + 1) * (1.0 - refl));
                    } else {
                        refl = 1.0; // Total internal reflection
                    }
                }

                if refl > 0.0 {
                    let reflected_dir = reflect(d, n);
                    sum = sum + trace(center + n * BIAS, reflected_dir, depth + 1) * refl;
                }
            }
            return sum * beer_lambert(r.absorption, t);
        }
        t += r.sd * sign;
    }
    BLACK
}

pub fn sample(o: Vec2, sample_count: usize) -> Color {
    let sum = (0..sample_count)
        .into_par_iter()
        .fold(
            || BLACK,
            |acc, i| {
                let mut rng = rand::rng();
                let a = TWO_PI * (i as f64 + rng.random::<f64>()) / (sample_count as f64);
                let color = trace(o, Vec2::new(a.cos(), a.sin()), 0);
                acc + color
            },
        )
        .reduce(|| BLACK, |a, b| a + b);
    sum * (1.0 / sample_count as f64)
}
