mod ray;
mod math;

use crate::math::Rgb;
use crate::ray::Ray;

fn main()
{
    // let col : Ray = Ray::new(Vec3 {x: 1.0, y: 1.0, z: 1.0}, Vec3 {x: 0.0, y: 0.0, z: 0.0});
    let nx : u32 = 200;
    let ny : u32 = 100;
    println!("P3\n{0} {1}\n255\n", nx, ny);
    for j in (0..ny).rev()
    {
        for i in 0..nx
        {
            let col : Rgb = Rgb {r: (i as f64)/(nx as f64), g: (j as f64)/(ny as f64), b: 0.2};
            let ir = (255.99 * col.r) as u32;
            let ig = (255.99 * col.g) as u32;
            let ib = (255.99 * col.b) as u32;
            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
