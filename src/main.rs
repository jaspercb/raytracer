extern crate raytracer;
extern crate rand;

use rand::Rng;
use rand::prelude::ThreadRng;

use raytracer::math::{Vec3, Rgb};
use raytracer::ray::Ray;
use raytracer::hittablelist::HittableList;
use raytracer::camera::Camera;
use raytracer::primitives::Sphere;

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut x;
    let mut y;
    let mut z;
    loop {
        x = 2.0*(rng.gen::<f64>() - 0.5);
        y = 2.0*(rng.gen::<f64>() - 0.5);
        z = 2.0*(rng.gen::<f64>() - 0.5);
        if (x*x + y*y *z*z) < 1.0
        {
            break;
        }
    }
    return Vec3{x: x, y: y, z:z};
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut hl: HittableList = HittableList::new();
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5}));
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0}));

    fn color(hl: &HittableList, r: &Ray, mut rng: &mut ThreadRng) -> Rgb
    {
        let mrh = hl.hit(&r, 0.0001, 99999999999.);
        match mrh
        {
            Some(rh) => {
                let target: Vec3 = rh.p + rh.normal + random_in_unit_sphere(&mut rng);
                return 0.5 * color(&hl, &Ray::new(rh.p, target), &mut rng);
                // let normal = rh.normal;
                // return 0.5 * Rgb {r: normal.x + 1.0, g: normal.y + 1.0, b: normal.z + 1.0};
            },
            _ => (),
        }
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0-t) * Rgb {r: 1.0, g: 1.0, b: 1.0} + t * Rgb {r: 0.5, g: 0.7, b: 1.0};
    }

    let nx : u32 = 200;
    let ny : u32 = 100;

    let cam: Camera = Camera::new();
    let nsamples = 10;
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
                col += color(&hl, &r, &mut rng); // / (nsamples as f64);
            }
            col /= nsamples as f64;
            col = col.sqrt();
            println!("{0} {1} {2}", (255.99 * col.r) as u32, (255.99 * col.g) as u32, (255.99 * col.b) as u32);
        }
    }
}
