use crate::ambient::Ambient;

use std::fs;

use serde::Deserialize;

use crate::{
    Color, Shape, Vec2,
    color::BLACK,
    sdf::{
        BoxShape, Capsule, Circle, Complement, Intersect, Ngon, Offset, Plane, Subtract, Triangle,
        Union,
    },
};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ShapeNode {
    Circle {
        center: [f64; 2],
        radius: f64,
        #[serde(default)]
        material: Material,
    },
    Plane {
        point: [f64; 2],
        normal: [f64; 2],
        #[serde(default)]
        material: Material,
    },
    Ngon {
        center: [f64; 2],
        radius: f64,
        n: f64,
        #[serde(default)]
        material: Material,
    },
    Capsule {
        a: [f64; 2],
        b: [f64; 2],
        radius: f64,
        #[serde(default)]
        material: Material,
    },
    Box {
        center: [f64; 2],
        theta: f64,
        half_size: [f64; 2],
        #[serde(default)]
        material: Material,
    },
    Triangle {
        a: [f64; 2],
        b: [f64; 2],
        c: [f64; 2],
        #[serde(default)]
        material: Material,
    },
    Union {
        children: Vec<ShapeNode>,
    },
    Intersect {
        children: Vec<ShapeNode>,
    },
    Subtract {
        a: Box<ShapeNode>,
        b: Box<ShapeNode>,
    },
    Offset {
        shape: Box<ShapeNode>,
        distance: f64,
    },
    Complement {
        shape: Box<ShapeNode>,
    },
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Material {
    #[serde(default)]
    pub reflectivity: f64,
    #[serde(default)]
    pub eta: f64,
    #[serde(default)]
    pub emissive: Option<[f64; 3]>,
    #[serde(default)]
    pub absorption: Option<[f64; 3]>,
    #[serde(default)]
    pub emissive_color: Option<[u8; 3]>,
    #[serde(default)]
    pub absorption_color: Option<[u8; 3]>,
}

impl Material {
    fn emissive_color(&self) -> Color {
        if let Some(c) = self.emissive {
            Color::new(c[0], c[1], c[2])
        } else if let Some(c) = self.emissive_color {
            let scale = 10.0 / 255.0;
            Color::new(
                c[0] as f64 * scale,
                c[1] as f64 * scale,
                c[2] as f64 * scale,
            )
        } else {
            BLACK
        }
    }
    fn absorption_color(&self) -> Color {
        if let Some(c) = self.absorption {
            Color::new(c[0], c[1], c[2])
        } else if let Some(c) = self.absorption_color {
            let scale = 3.0 / 255.0;
            Color::new(
                (255 - c[0]) as f64 * scale,
                (255 - c[1]) as f64 * scale,
                (255 - c[2]) as f64 * scale,
            )
        } else {
            BLACK
        }
    }
}

impl ShapeNode {
    pub fn build(&self) -> Box<dyn Shape> {
        match self {
            ShapeNode::Circle {
                center,
                radius,
                material,
            } => {
                let c = Vec2::new(center[0], center[1]);
                Box::new(
                    Circle::new(c, *radius)
                        .with_reflectivity(material.reflectivity)
                        .with_eta(material.eta)
                        .with_emissive(material.emissive_color())
                        .with_absorption(material.absorption_color()),
                )
            }
            ShapeNode::Plane {
                point,
                normal,
                material,
            } => {
                let p = Vec2::new(point[0], point[1]);
                let n = Vec2::new(normal[0], normal[1]);
                Box::new(
                    Plane::new(p, n)
                        .with_reflectivity(material.reflectivity)
                        .with_eta(material.eta)
                        .with_emissive(material.emissive_color())
                        .with_absorption(material.absorption_color()),
                )
            }
            ShapeNode::Ngon {
                center,
                radius,
                n,
                material,
            } => {
                let c = Vec2::new(center[0], center[1]);
                Box::new(
                    Ngon::new(c, *radius, *n)
                        .with_reflectivity(material.reflectivity)
                        .with_eta(material.eta)
                        .with_emissive(material.emissive_color())
                        .with_absorption(material.absorption_color()),
                )
            }
            ShapeNode::Capsule {
                a,
                b,
                radius,
                material,
            } => {
                let pa = Vec2::new(a[0], a[1]);
                let pb = Vec2::new(b[0], b[1]);
                Box::new(
                    Capsule::new(pa, pb, *radius)
                        .with_reflectivity(material.reflectivity)
                        .with_eta(material.eta)
                        .with_emissive(material.emissive_color())
                        .with_absorption(material.absorption_color()),
                )
            }
            ShapeNode::Box {
                center,
                theta,
                half_size,
                material,
            } => {
                let c = Vec2::new(center[0], center[1]);
                let h = Vec2::new(half_size[0], half_size[1]);
                Box::new(
                    BoxShape::new(c, *theta, h)
                        .with_reflectivity(material.reflectivity)
                        .with_eta(material.eta)
                        .with_emissive(material.emissive_color())
                        .with_absorption(material.absorption_color()),
                )
            }
            ShapeNode::Triangle { a, b, c, material } => {
                let pa = Vec2::new(a[0], a[1]);
                let pb = Vec2::new(b[0], b[1]);
                let pc = Vec2::new(c[0], c[1]);
                Box::new(
                    Triangle::new(pa, pb, pc)
                        .with_reflectivity(material.reflectivity)
                        .with_eta(material.eta)
                        .with_emissive(material.emissive_color())
                        .with_absorption(material.absorption_color()),
                )
            }
            ShapeNode::Union { children } => {
                let mut iter = children.iter();
                let first = iter.next().unwrap().build();
                iter.fold(first, |acc, node| Box::new(Union::new(acc, node.build())))
            }
            ShapeNode::Intersect { children } => {
                let mut iter = children.iter();
                let first = iter.next().unwrap().build();
                iter.fold(first, |acc, node| {
                    Box::new(Intersect::new(acc, node.build()))
                })
            }
            ShapeNode::Subtract { a, b } => Box::new(Subtract::new(a.build(), b.build())),
            ShapeNode::Offset { shape, distance } => {
                Box::new(Offset::new(shape.build(), *distance))
            }
            ShapeNode::Complement { shape } => Box::new(Complement::new(shape.build())),
        }
    }
}

#[derive(Deserialize)]
pub struct SceneFile {
    #[serde(default)]
    pub ambient: Ambient,
    pub shape: ShapeNode,
}

pub fn load_scene_from_file(path: &str) -> (Box<dyn Shape>, Ambient) {
    let content = fs::read_to_string(path).expect("Failed to read scene file");
    let value: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON syntax");

    if value.get("shape").is_some() {
        let sf: SceneFile = serde_json::from_value(value).expect("Invalid scene file format");
        (sf.shape.build(), sf.ambient)
    } else {
        let node: ShapeNode = serde_json::from_str(&content).expect("Failed to parse JSON");
        (node.build(), Ambient::default())
    }
}
