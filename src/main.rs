mod ray;
mod math;

use crate::math::{Vec3, Rgb};
use crate::ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64
{
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0;
    }
    return (-b - discriminant.sqrt()) / (2.0*a);
}

fn color(r: Ray) -> Rgb
{
    let t = hit_sphere(Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5, &r);
    if t > 0.0
    {
        let N: Vec3 = (r.at(t) - Vec3 {x: 0.0, y: 0.0, z: -1.0}).normalized();
        return 0.5 * Rgb {r: N.x + 1.0, g: N.y + 1.0, b: N.z + 1.0};
    }
    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t) * Rgb {r: 1.0, g: 1.0, b: 1.0} + t * Rgb {r: 0.5, g: 0.7, b: 1.0};
}

fn main()
{
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
            let col = color(r);
            let ir = (255.99 * col.r) as u32;
            let ig = (255.99 * col.g) as u32;
            let ib = (255.99 * col.b) as u32;
            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
