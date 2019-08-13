extern crate raytracer;
extern crate rand;

use rand::Rng;

use raytracer::math::{Vec3, Rgb};
use raytracer::ray::Ray;
use raytracer::hittablelist::HittableList;
use raytracer::camera::Camera;
use raytracer::primitives::Sphere;
use raytracer::material::lambertian::Lambertian;
use raytracer::material::metal::Metal;
use raytracer::material::dielectric::Dielectric;

fn main() {
    let mut rng = rand::thread_rng();

    let mut hl: HittableList = HittableList::new();
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5, mat: Box::new(Lambertian::new(Rgb {r: 0.8, g: 0.3, b: 0.3}))}));
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0, mat: Box::new(Lambertian::new(Rgb {r: 0.8, g: 0.8, b: 0.0}))}));
    hl.push(Box::new(Sphere{center: Vec3{x: 1.0, y: 0.0, z: -1.0}, radius: 0.5, mat: Box::new(Metal::new(Rgb {r: 0.8, g: 0.6, b: 0.2}, 0.3))}));
    // hl.push(Box::new(Sphere{center: Vec3{x: -1.0, y: 0.0, z: -1.0}, radius: 0.5, mat: Box::new(Metal::new(Rgb {r: 0.8, g: 0.8, b: 0.8}, 1.0))}));
    hl.push(Box::new(Sphere{center: Vec3{x: -1.0, y: 0.0, z: -1.0}, radius: 0.5, mat: Box::new(Dielectric::new(1.5))}));

    fn color(hl: &HittableList, r: &Ray, depth: u8) -> Rgb
    {
        if depth >= 50 { return Rgb::zero(); };

        let mrh = hl.hit(&r, 0.0001, std::f64::MAX);
        match mrh
        {
            Some(rh) => {
                match rh.mat_ptr.scatter(&r, &rh)
                {
                    Some((reflected, attenuation)) => {
                        return attenuation * color(&hl, &reflected, depth+1);
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

    let nx : u32 = 200;
    let ny : u32 = 100;

    let cam: Camera = Camera::new(Vec3::new(-2.0, 2.0, 1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, (nx as f64)/(ny as f64));
    let nsamples = 20;
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
                col += color(&hl, &r, 0); // / (nsamples as f64);
            }
            col /= nsamples as f64;
            col = col.sqrt();
            println!("{0} {1} {2}", (255.99 * col.r) as u32, (255.99 * col.g) as u32, (255.99 * col.b) as u32);
        }
    }
}
