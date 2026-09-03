use crate::{sdf::TWO_PI, vec2::Vec2};

pub fn box_sdf(o: Vec2, c: Vec2, theta: f64, s: Vec2) -> f64 {
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

pub fn circle_sdf(center: Vec2, point: Vec2, radius: f64) -> f64 {
    (point - center).length() - radius
}

pub fn plane_sdf(o: Vec2, p: Vec2, normal: Vec2) -> f64 {
    (o - p).dot(&normal)
}

pub fn ngon_sdf(o: Vec2, c: Vec2, r: f64, n: f64) -> f64 {
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

pub fn capsule_sdf(o: Vec2, a: Vec2, b: Vec2, r: f64) -> f64 {
    segment_sdf(o, a, b) - r
}

pub fn triangle_sdf(o: Vec2, a: Vec2, b: Vec2, c: Vec2) -> f64 {
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

pub fn segment_sdf(o: Vec2, a: Vec2, b: Vec2) -> f64 {
    let v = o - a;
    let u = b - a;

    let t = (v.dot(&u) / u.dot(&u)).min(1.0).clamp(0.0, 1.0);

    (v - (u * t)).length()
}
