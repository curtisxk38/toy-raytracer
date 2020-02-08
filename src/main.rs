extern crate image;

use std::env;
use std::ptr;

mod parse;
mod lib;

use crate::lib::Sphere;
use crate::lib::Sun;
use crate::lib::Ray;
use crate::lib::Vector3;
use crate::lib::Color;
use crate::lib::Bulb;


fn clamp(value: f64) -> f64 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

struct Raytracer {
    width: u32,
    height: u32,
    imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    spheres: Vec<Sphere>,
    suns: Vec<Sun>,
    bulbs: Vec<Bulb>,
    eye: Vector3,
    forward: Vector3,
    right: Vector3,
    up: Vector3,
}

impl Raytracer {
    fn new(width: u32, height: u32, spheres: Vec<Sphere>, suns: Vec<Sun>, bulbs: Vec<Bulb>) -> Raytracer {
        Raytracer { width: width, height: height,
            imgbuf: image::ImageBuffer::new(width, height),
            spheres: spheres,
            suns: suns,
            bulbs: bulbs,
            eye: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            forward: Vector3 {x: 0.0, y: 0.0, z: -1.0},
            right: Vector3 {x: 1.0, y: 0.0, z: 0.0},
            up: Vector3 {x: 0.0, y: 1.0, z: 0.0},
        }
    }

    fn save(self, filename: &str) {
        self.imgbuf.save(filename).unwrap();
    }

    fn get_ray_from_pixel(&self, x: f64, y: f64) -> Ray {
        

        let float_w = f64::from(self.width);
        let float_h = f64::from(self.height);

        let max_dim = float_h.max(float_w);
        let sx = (2.0 * x - float_w) / max_dim;
        let sy = (float_h - 2.0 * y) / max_dim;

        let dir = self.up.scale(sy);
        let dir = dir.add(&(self.right.scale(sx)));
        let dir = dir.add(&self.forward);

        Ray::new(self.eye.clone(), dir)
    }

    // go thru spheres and check if the camera is inside them
    //  we need to know this, in order to flip the normals of spheres that contain the camera
    fn check_spheres(&mut self) {
        for sphere in &mut self.spheres {
            // camera is located at eye
            let dist = self.eye.subtract(&sphere.center);
            if dist.magnitude() < sphere.r {
                sphere.contains_camera = true;
            }
        }
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

    // originating shape = shape the shadow ray is coming from
    //  shadow ray shouldn't intersect the shape it came from
    //  but it may if we don't explicitly check (due to float imprecision)
    fn is_in_sun_shadow(&self, originating_shape: &Sphere, col_point: &Vector3, sun: &Sun) -> bool {
        //return false;
        for sphere in &self.spheres {
            let shadow_ray = Ray::new(col_point.clone(), sun.direction.clone());
            if !ptr::eq(sphere, originating_shape) && sphere.intersect(&shadow_ray) >= 0.0 {
				return true;
			}
        }
        return false;
    }

    fn is_in_bulb_shadow(&self, originating_shape: &Sphere, col_point: &Vector3, bulb: &Bulb) -> bool {
        //return false;
        let to_bulb = bulb.position.subtract(col_point);
		let dist_to_bulb = to_bulb.magnitude();
		let shadow_ray = Ray::new(col_point.clone(), to_bulb.clone());
		for sphere in &self.spheres {
            if !ptr::eq(sphere, originating_shape) {
                let intersect = sphere.intersect(&shadow_ray);
                // if there is an intersection, and the intersection is in between the bulb and the shadow ray origin
                if intersect >= 0.0 && intersect < dist_to_bulb {
                    return true;
                }
            }

		}
		return false;
    }

    fn shoot_ray(&self, ray: Ray, _level: u32) -> Color {
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
        let mut normal = min_shape.normal(&collision_point);
        
        // flip normal if camera is inside sphere
        //  since we want the inner surface of the sphere
        if min_shape.contains_camera {
            normal = normal.scale(-1.0); 
        }

        for sun in &self.suns {
            if !self.is_in_sun_shadow(&min_shape, &collision_point, &sun) {
                let intensity = clamp(normal.dot(&sun.direction));
                let mut diffuse_color = min_shape.color.mul(&sun.color).scale(intensity);
                diffuse_color.a = 1.0;
                color = color.add(&diffuse_color);
           }
        }

        for bulb in &self.bulbs {
            if !self.is_in_bulb_shadow(&min_shape, &collision_point, &bulb) {
                let to_bulb = bulb.position.subtract(&collision_point);
                let intensity = clamp(normal.dot(&to_bulb.normalize()));
                let mut diffuse_color = min_shape.color.mul(&bulb.color).scale(intensity);
                // scale illumination based on 1 over distance between squared
                diffuse_color = diffuse_color.scale(1.0 / to_bulb.dot(&to_bulb));
                diffuse_color.a = 1.0;
                color = color.add(&diffuse_color);	
            }
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

    let mut r = Raytracer::new(img.cfg.width, img.cfg.height, img.spheres, img.suns, img.bulbs);
    r.check_spheres();
    r.trace_from_camera();
    r.save(&img.cfg.filename);
}
