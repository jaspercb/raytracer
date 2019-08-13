extern crate raytracer;

use raytracer::math::{Vec3, Rgb};
use raytracer::ray::Ray;
use raytracer::hittable::Hittable;
use raytracer::hittablelist::HittableList;

use raytracer::primitives::Sphere;

fn main()
{
    let mut hl: HittableList = HittableList::new();
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5}));
    hl.push(Box::new(Sphere{center: Vec3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0}));
    fn color(hl: &HittableList, r: Ray) -> Rgb
    {
        let mrh = hl.hit(&r, 0.0, 99999999999.);
        match mrh
        {
            Some(rh) => {
                let normal = rh.normal;
                return 0.5 * Rgb {r: normal.x + 1.0, g: normal.y + 1.0, b: normal.z + 1.0};
            },
            _ => (),
        }
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0-t) * Rgb {r: 1.0, g: 1.0, b: 1.0} + t * Rgb {r: 0.5, g: 0.7, b: 1.0};
    }

    // let col : Ray = Ray::new(Vec3 {x: 1.0, y: 1.0, z: 1.0}, Vec3 {x: 0.0, y: 0.0, z: 0.0});
    let nx : u32 = 200;
    let ny : u32 = 100;

    let lower_left_corner = Vec3 {x: -2.0, y: -1.0, z: -1.0};
    let horizontal = Vec3 {x: 4.0, y: 0.0, z: 0.0};
    let vertical = Vec3 {x: 0.0, y: 2.0, z: 0.0};
    let origin = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    println!("P3\n{0} {1}\n255\n", nx, ny);
    for j in (0..ny).rev()
    {
        for i in 0..nx
        {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = color(&hl, r);
            let ir = (255.99 * col.r) as u32;
            let ig = (255.99 * col.g) as u32;
            let ib = (255.99 * col.b) as u32;
            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
