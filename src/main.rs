extern crate raytracer; extern crate rand;

use rand::Rng;

use raytracer::math::{Vec3, Rgb};
use raytracer::ray::Ray;
use raytracer::hittable::Hittable;
use raytracer::camera::Camera;
use raytracer::primitives::Sphere;
use raytracer::material::lambertian::Lambertian;
use raytracer::material::metal::Metal;
use raytracer::material::dielectric::Dielectric;
use raytracer::texture::{ConstantTexture, CheckerTexture, NoiseTexture};
use raytracer::bvh::BvhNode;
use raytracer::util::Perlin;

fn basic_scene() -> Box<dyn Hittable> {
    let mut hl: Vec<Box<dyn Hittable>> = Vec::new();
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5, mat: Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb::new(0.8, 0.3, 0.3)))))}));
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0, mat: Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb {r: 0.8, g: 0.8, b: 0.0}))))}));
    hl.push(Box::new(Sphere{center: Vec3{x: 1.0, y: 0.0, z: -1.0}, radius: 0.5, mat: Box::new(Metal::new(Rgb {r: 0.8, g: 0.6, b: 0.2}, 0.3))}));
    hl.push(Box::new(Sphere{center: Vec3{x: -1.0, y: 0.0, z: -1.0}, radius: 0.5, mat: Box::new(Dielectric::new(1.5))}));
    return BvhNode::construct(&mut hl, 0.0, 0.0);
}

fn big_scene() -> Box<dyn Hittable> {
    let mut hl: Vec<Box<dyn Hittable>> = Vec::new();
    let mut rng = rand::thread_rng();

    // ground
    hl.push(Box::new(
            Sphere{center: Vec3{x: 0.0, y: -1000.0, z: -0.0}, radius: 1000.0,
            mat: Box::new(Lambertian::new(
                Box::new(NoiseTexture::new(Perlin::new(), 5.0))
            ))
        }
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new((a as f64) + 0.9 * rng.gen::<f64>(), 0.2, (b as f64) + 0.9*rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    hl.push(Box::new(Sphere{center, radius: 0.2, mat: Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb::new(rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>())))))}));
                } else if choose_mat < 0.95 { // metal
                    hl.push(Box::new(Sphere{center, radius: 0.2, mat: Box::new(Metal::new(Rgb::new(0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>()), 0.5 * (1.0 + rng.gen::<f64>())), 0.5 * rng.gen::<f64>()))}));
                } else {
                    hl.push(Box::new(Sphere{center, radius: 0.2, mat: Box::new(Dielectric::new(1.5))}));
                }
            }
        }
    }
    hl.push(Box::new(Sphere{center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, mat: Box::new(Lambertian::new(Box::new(ConstantTexture::new(Rgb::new(0.4, 0.2, 0.1)))))}));
    hl.push(Box::new(Sphere{center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, mat: Box::new(Dielectric::new(1.5))}));
    hl.push(Box::new(Sphere{center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, mat: Box::new(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0))}));

    return BvhNode::construct(&mut hl, 0.0, 0.0);
}

fn main() {
    let mut rng = rand::thread_rng();
    
    // let hl = basic_scene();
    let hl = big_scene();

    fn color(hl: &Hittable, r: &Ray, depth: u8) -> Rgb
    {
        if depth >= 50 { return Rgb::zero(); };

        let mrh = hl.hit(&r, 0.0001, std::f64::MAX);
        match mrh
        {
            Some(rh) => {
                match rh.mat_ptr.scatter(&r, &rh)
                {
                    Some((reflected, attenuation)) => {
                        return attenuation * color(hl, &reflected, depth+1);
                    }
                    _ => {
                        return Rgb::zero();
                    }
                }
                // let normal = rh.normal;
                // return 0.5 * Rgb {r: normal.x + 1.0, g: normal.y + 1.0, b: normal.z + 1.0};
            },
            _ => (),
        }
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0-t) * Rgb {r: 1.0, g: 1.0, b: 1.0} + t * Rgb {r: 0.5, g: 0.7, b: 1.0};
    }

    let nx : u32 = 300;
    let ny : u32 = 200;

    let lookfrom = Vec3::new(5.0, 1.0, 2.0);
    let lookat = Vec3::new(0.0, 0.2, 0.0);
    let dist_to_focus = (lookat - lookfrom).magnitude();
    let aperture = 0.1;
    let cam: Camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 30.0, (nx as f64)/(ny as f64), aperture, dist_to_focus);
    let nsamples = 50;
    println!("P3\n{0} {1}\n255\n", nx, ny);
    for j in (0..ny).rev()
    {
        for i in 0..nx
        {
            let mut col = Rgb{r: 0.0, g: 0.0, b: 0.0};
            for _ in 0..nsamples
            {
                let u = (i as f64 + rng.gen::<f64>()) / (nx as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (ny as f64);
                let r: Ray = cam.get_ray(u, v);
                col += color(&*hl, &r, 0);
            }
            col /= nsamples as f64;
            col = col.sqrt();
            println!("{0} {1} {2}", (255.99 * col.r) as u32, (255.99 * col.g) as u32, (255.99 * col.b) as u32);
        }
    }
}
