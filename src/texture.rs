extern crate image;

use crate::texture::image::GenericImageView;
use crate::texture::image::Pixel;

use crate::math::{Rgb, Uv, Vec3};
use crate::util::Perlin;

pub trait Texture: core::fmt::Debug {
    fn value(&self, uv: &Uv, p: &Vec3) -> Rgb;
}

#[derive(Debug)]
pub struct ConstantTexture {
    color: Rgb,
}

impl ConstantTexture {
    pub fn new(color: Rgb) -> ConstantTexture {
        Self { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _uv: &Uv, _p: &Vec3) -> Rgb {
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
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: &Uv, p: &Vec3) -> Rgb {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        return (if sines < 0.0 { &self.odd } else { &self.even }).value(uv, p);
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: f64) -> NoiseTexture {
        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: &Uv, p: &Vec3) -> Rgb {
        return Rgb::new(1.0, 1.0, 1.0) * self.noise.noise(&(self.scale * *p));
    }
}

pub struct ImageTexture {
    image: image::DynamicImage,
}

impl ImageTexture {
    pub fn new(image: image::DynamicImage) -> ImageTexture {
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: &Uv, _p: &Vec3) -> Rgb {
        let (nx, ny) = self.image.dimensions();
        let i = ((uv.u * nx as f64) as u32).max(0).min(nx - 1);
        let j = (((1.0 - uv.v) * (ny as f64) - 0.001) as u32)
            .max(0)
            .min(ny - 1);
        let rgb = self.image.get_pixel(i, j).to_rgb();
        return Rgb::new(
            (rgb[0] as f64) / 255.0,
            (rgb[1] as f64) / 255.0,
            (rgb[2] as f64) / 255.0,
        );
    }
}

impl std::fmt::Debug for ImageTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ImageTexture>")
    }
}
