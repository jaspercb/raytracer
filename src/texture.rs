use crate::math::{Rgb, Vec3};
use crate::util::Perlin;

pub trait Texture: core::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Rgb;
}

#[derive(Debug)]
pub struct ConstantTexture {
    color: Rgb,
}

impl ConstantTexture {
    pub fn new(color: Rgb) -> ConstantTexture {
        Self {color}
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Rgb {
        self.color
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> CheckerTexture {
        Self {odd, even}
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Rgb {
        let sines = (10.0*p.x).sin() * (10.0*p.y).sin() * (10.0*p.z).sin();
        return (if sines < 0.0 {&self.odd} else {&self.even}).value(u, v, p)
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: f64) -> NoiseTexture {
        Self {noise, scale}
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Rgb {
        return Rgb::new(1.0, 1.0, 1.0) * self.noise.noise(&(self.scale * p));
    }
}
