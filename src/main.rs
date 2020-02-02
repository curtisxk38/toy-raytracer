extern crate image;

use std::env;


mod parse;
mod lib;

use crate::lib::Sphere;
use crate::lib::Sun;
use crate::lib::Ray;
use crate::lib::Vector3;
use crate::lib::Color;



struct Raytracer {
    width: u32,
    height: u32,
    imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    spheres: Vec<Sphere>,
    suns: Vec<Sun>
}

impl Raytracer {
    fn new(width: u32, height: u32, spheres: Vec<Sphere>, suns: Vec<Sun>) -> Raytracer {
        Raytracer { width: width, height: height,
            imgbuf: image::ImageBuffer::new(width, height),
            spheres: spheres,
            suns: suns
        }
    }

    fn save(self, filename: &str) {
        self.imgbuf.save(filename).unwrap();
    }

    fn get_ray_from_pixel(&mut self, x: f64, y: f64) -> Ray {
        let eye = Vector3 {x: 0.0, y: 0.0, z: 0.0};
        let forward = Vector3 {x: 0.0, y: 0.0, z: -1.0};
        let right = Vector3 {x: 1.0, y: 0.0, z: 0.0};
        let up = Vector3 {x: 0.0, y: 1.0, z: 0.0};

        let float_w = f64::from(self.width);
        let float_h = f64::from(self.height);

        let max_dim = float_h.max(float_w);
        let sx = (2.0 * x - float_w) / max_dim;
        let sy = (float_h - 2.0 * y) / max_dim;

        let dir = up.scale(sy);
        let dir = dir.add(&(right.scale(sx)));
        let dir = dir.add(&forward);

        return Ray {origin: eye, direction: dir.normalize()};

    }

    fn trace_from_camera(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let ray = self.get_ray_from_pixel(f64::from(i), f64::from(j));
                let color = self.shoot_ray(ray, 0);
                let color_bytes = color.to_bytes_color();
                let pixel = self.imgbuf.get_pixel_mut(i, j);
                *pixel = color_bytes;
            }
        }
    }

    fn shoot_ray(&mut self, ray: Ray, _level: u32) -> Color {
        let mut min_dist = self.spheres[0].intersect(&ray);
        let mut min_shape = &self.spheres[0];

        for sphere in &self.spheres[1..] {
            let intersect_dist = sphere.intersect(&ray);
            if intersect_dist > 0.0 && (intersect_dist < min_dist || min_dist < 0.0) {
                min_dist = intersect_dist;
                min_shape = &sphere;
            }
        }

        if min_dist < 0.0 {
            return Color {r: 0.0, g: 0.0, b: 0.0, a: 0.0};
        }

        let mut color = Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0};

        // lambertian reflectance
        let collision_point = ray.origin.add(&ray.direction.scale(min_dist));
        let normal = min_shape.normal(collision_point);

        for sun in &self.suns {
            let mut diffuse_color = min_shape.color.mul(&sun.color).scale(normal.dot(&sun.direction));
            diffuse_color.a = 1.0;
            color = color.add(&diffuse_color);
        }

        return color;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }
    let filename = &args[1];
    let img = parse::parse(filename);

    let mut r = Raytracer::new(img.cfg.width, img.cfg.height, img.spheres, img.suns);
    r.trace_from_camera();
    r.save(&img.cfg.filename);
}
