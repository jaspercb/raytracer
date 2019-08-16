extern crate image;
extern crate rand;
extern crate rayon;
extern crate raytracer;

use rand::Rng;
use rayon::prelude::*;
use std::path::Path;

use raytracer::bvh::BvhNode;
use raytracer::camera::Camera;
use raytracer::hittable::Hittable;
use raytracer::material::dielectric::Dielectric;
use raytracer::material::diffuselight::DiffuseLight;
use raytracer::material::lambertian::Lambertian;
use raytracer::material::metal::Metal;
use raytracer::math::{Rgb, Vec3};
use raytracer::primitives::Sphere;
use raytracer::ray::Ray;
use raytracer::texture::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use raytracer::util::Perlin;

fn basic_scene() -> Box<dyn Hittable> {
    let mut hl: Vec<Box<dyn Hittable>> = Vec::new();
    hl.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb::new(
            0.8, 0.3, 0.3,
        ))))),
    )));
    hl.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb::new(
            0.8, 0.8, 0.0,
        ))))),
    )));
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: Box::new(Metal::new(
            Rgb {
                r: 0.8,
                g: 0.6,
                b: 0.2,
            },
            0.3,
        )),
    }));
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: Box::new(Dielectric::new(1.5)),
    }));
    return BvhNode::construct(&mut hl, 0.0, 0.0);
}

fn light_scene() -> Box<dyn Hittable> {
    let mut hl: Vec<Box<dyn Hittable>> = Vec::new();
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb::new(
            0.8, 0.3, 0.3,
        ))))),
    }));
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        mat: Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb {
            r: 0.8,
            g: 0.8,
            b: 0.0,
        })))),
    }));
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: Box::new(Metal::new(
            Rgb {
                r: 0.8,
                g: 0.6,
                b: 0.2,
            },
            0.3,
        )),
    }));
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: Box::new(Dielectric::new(1.5)),
    }));
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.5,
            z: -2.0,
        },
        radius: 0.5,
        mat: Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(Rgb::new(
            8.0, 8.0, 8.0,
        ))))),
    }));
    return BvhNode::construct(&mut hl, 0.0, 0.0);
}

fn big_scene() -> Box<dyn Hittable> {
    let mut hl: Vec<Box<dyn Hittable>> = Vec::new();
    let mut rng = rand::thread_rng();

    // ground
    hl.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: -0.0,
        },
        radius: 1000.0,
        mat: Box::new(Lambertian::new(
            Box::new(NoiseTexture::new(Perlin::new(), 5.0)), // Box::new(ImageTexture::new(image::open(&Path::new("earth.jpg")).unwrap()))
        )),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.2,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    hl.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))))),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    hl.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Box::new(Metal::new(
                            Rgb::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        )),
                    }));
                } else {
                    hl.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Box::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }
    hl.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Box::new(Lambertian::new(Box::new(ImageTexture::new(
            image::open(&Path::new("earth.jpg")).unwrap(),
        )))),
    }));
    hl.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Box::new(Lambertian::new(Box::new(ImageTexture::new(
            image::open(&Path::new("earth.jpg")).unwrap(),
        )))),
    }));
    // hl.push(Box::new(Sphere{center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, mat: Box::new(Dielectric::new(1.5))}));
    //hl.push(Box::new(Sphere{center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, mat: Box::new(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0))}));

    return BvhNode::construct(&mut hl, 0.0, 0.0);
}

fn main() {
    let mut rng = rand::thread_rng();

    // let hl = basic_scene();
    // let hl = big_scene();
    let hl = light_scene();

    fn color(hl: &Hittable, r: &Ray, depth: u8) -> Rgb {
        if depth >= 50 {
            return Rgb::zero();
        };

        match hl.hit(&r, 0.0001, std::f64::MAX) {
            Some(hr) => {
                let base: Rgb = match hr.mat_ptr.scatter(&r, &hr) {
                    Some((reflected, attenuation)) => {
                        attenuation * color(hl, &reflected, depth + 1)
                    }
                    _ => Rgb::zero(),
                };
                return hr.mat_ptr.emitted(&hr.uv, &hr.p) + base;
            }
            _ => (),
        }
        return Rgb::zero();
    }

    let nx = 300;
    let ny = 150;
    let nsamples = 100;

    let lookfrom = Vec3::new(5.0, 1.0, 2.0);
    let lookat = Vec3::new(0.0, 0.2, 0.0);
    let dist_to_focus = (lookat - lookfrom).magnitude();
    let aperture = 0.1;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        (nx as f64) / (ny as f64),
        aperture,
        dist_to_focus,
    );
    println!("P3\n{0} {1}\n255\n", nx, ny);
    let mut pixels: Vec<(u32, u32)> = Vec::new();
    for j in (0..ny).rev() {
        // if (j % 10 == 0) { eprintln!("{}/{}", j, ny); }
        for i in 0..nx {
            pixels.push((i, j));
        }
    }
    for col in pixels
        .par_iter()
        .map(|(i, j)| -> Rgb {
            let mut col = Rgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
            let mut trng = rand::thread_rng();
            for _ in 0..nsamples {
                let u = (*i as f64 + trng.gen::<f64>()) / (nx as f64);
                let v = (*j as f64 + trng.gen::<f64>()) / (ny as f64);
                let r: Ray = cam.get_ray(u, v);
                col += color(&*hl, &r, 0);
            }
            col /= nsamples as f64;
            return col.sqrt();
        })
        .collect::<Vec<Rgb>>()
    {
        println!(
            "{0} {1} {2}",
            (255.99 * col.r.min(1.0).max(0.0)) as u32,
            (255.99 * col.g.min(1.0).max(0.0)) as u32,
            (255.99 * col.b.min(1.0).max(0.0)) as u32
        );
    }
}
