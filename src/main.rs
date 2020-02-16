extern crate image;

use std::env;

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
    bounces: i32,
    eye: Vector3,
    forward: Vector3,
    right: Vector3,
    up: Vector3,
}

impl Raytracer {
    fn new(width: u32, height: u32, spheres: Vec<Sphere>, suns: Vec<Sun>, bulbs: Vec<Bulb>, bounces: i32) -> Raytracer {
        Raytracer { width: width, height: height,
            imgbuf: image::ImageBuffer::new(width, height),
            spheres: spheres,
            suns: suns,
            bulbs: bulbs,
            bounces: bounces,
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

    fn trace_from_camera(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                println!("pixel: {}, {}", i, j);
                let ray = self.get_ray_from_pixel(f64::from(i), f64::from(j));
                let color = self.shoot_ray(ray, self.bounces);
                let color_bytes = color.to_bytes_color();
                let pixel = self.imgbuf.get_pixel_mut(i, j);
                *pixel = color_bytes;
            }
        }
    }

    // originating shape = shape the shadow ray is coming from
    //  shadow ray shouldn't intersect the shape it came from
    //  but it may if we don't explicitly check (due to float imprecision)
    fn is_in_sun_shadow(&self, col_point: &Vector3, sun: &Sun) -> bool {
        let shadow_ray = Ray::new(col_point.clone(), sun.direction.clone());
        for sphere in &self.spheres {
            if sphere.intersect(&shadow_ray) >= 0.0 {
				return true;
			}
        }
        return false;
    }

    fn is_in_bulb_shadow(&self, col_point: &Vector3, bulb: &Bulb) -> bool {
        let to_bulb = bulb.position.subtract(col_point);
		let dist_to_bulb = to_bulb.magnitude();
		let shadow_ray = Ray::new(col_point.clone(), to_bulb.clone());
		for sphere in &self.spheres {
            let intersect = sphere.intersect(&shadow_ray);
            // if there is an intersection, and the intersection is in between the bulb and the shadow ray origin
            if intersect >= 0.0 && intersect < dist_to_bulb {
                return true;
            }
		}
		return false;
    }

    fn shoot_ray(&self, ray: Ray, level: i32) -> Color {
        let bias = 0.0001;
        // stop bouncing new rays
        if level < 0 {
            return Color::transparent();
        }

        let mut min_dist = self.spheres[0].intersect(&ray);
        let mut min_shape = &self.spheres[0];

        for sphere in &self.spheres[1..] {
            let intersect_dist = sphere.intersect(&ray);
            if intersect_dist > 0.0 && (intersect_dist < min_dist || min_dist < 0.0) {
                min_dist = intersect_dist;
                min_shape = &sphere;
            }
        }

        // no collision
        if min_dist < 0.0 {
            return Color::transparent();
        }

        println!("lvl {}: {:?}", level, min_shape);

        let collision_point = ray.origin.add(&ray.direction.scale(min_dist));
        let mut normal = min_shape.normal(&collision_point);
        
        // If the normal and the view direction are not opposite to each other
        // reverse the normal direction. That also means we are inside the sphere so correct ior
        let mut ior = 1.458;
        if ray.direction.dot(&normal) > 0.0 {
            normal = normal.scale(-1.0); 
            ior = 1.0 / ior;
        }
        
        // shininess
        /*
        GLSL spec:
        For the incident vector I and surface orientation N, returns the reflection direction:I – 2 ∗dot(N, I) ∗ N
        N must already be normalized in order to achieve the desired result.
        */
        let mut shiny_color = Color::black();
        if min_shape.shininess.r != 0.0 {
            let temp_scale = 2.0 * normal.dot(&ray.direction);
            let scaled_n = normal.scale(temp_scale);
            let new_dir = ray.direction.subtract(&scaled_n);
            let collision_point = collision_point.add(&normal.scale(bias));
            let new_ray = Ray::new(collision_point, new_dir);
            println!("reflect");
            shiny_color = self.shoot_ray(new_ray, level - 1);
        }

        // transparency
        /*
        GLSL spec:
        For the incident vector I and surface normal N, and the ratio of indices of refraction eta, return the refraction vector.
        The result is computed by:
        k = 1.0 - eta * eta * (1.0 - dot(N, I) * dot(N, I))
        if (k < 0.0)
            return genType(0.0)
        else
            return eta * I - (eta * dot(N, I) + sqrt(k)) * N
        The input parameters for the incident vector I and the surface normal N must already be normalized to get the desired results.
        */
        let mut trans_color = Color::black();
        if min_shape.transparency.r != 0.0 {
            // index of refraction
            let eta = ior;
            let normal_dot_incident = normal.dot(&ray.direction);
            let k = 1.0 - eta * eta * (1.0 - normal_dot_incident * normal_dot_incident);
            let new_dir;
            if k < 0.0 {
                // we have total internal reflection:
                let temp_scale = 2.0 * normal_dot_incident;
                let scaled_n = normal.scale(temp_scale);
                println!("total internal reflect");
                new_dir = ray.direction.subtract(&scaled_n);
            } else {
                let temp_scale = eta * normal_dot_incident + k.sqrt();
                println!("refract");
                new_dir = ray.direction.scale(eta).subtract(&normal.scale(temp_scale)); 
            }
            let collision_point = collision_point.subtract(&normal.scale(bias));
            let new_ray = Ray::new(collision_point, new_dir);
            trans_color = self.shoot_ray(new_ray, level - 1);

            if trans_color.r != 0.0 {

            println!("{:?}", trans_color);
            }
        }

        // lambertian reflectance
        let mut diffuse_color = Color::black();

         {
            let collision_point = collision_point.add(&normal.scale(bias));
            for sun in &self.suns {
                if !self.is_in_sun_shadow(&collision_point, &sun) {
                    let intensity = clamp(normal.dot(&sun.direction));
                    let mut color_from_light = min_shape.color.mul(&sun.color).scale(intensity);
                    color_from_light.a = 1.0;
                    diffuse_color = diffuse_color.add(&color_from_light);
               }
            }
    
            for bulb in &self.bulbs {
                if !self.is_in_bulb_shadow(&collision_point, &bulb) {
                    let to_bulb = bulb.position.subtract(&collision_point);
                    let intensity = clamp(normal.dot(&to_bulb.normalize()));
                    let mut color_from_light = min_shape.color.mul(&bulb.color).scale(intensity);
                    // scale illumination based on 1 over distance between squared
                    color_from_light = color_from_light.scale(1.0 / to_bulb.dot(&to_bulb));
                    color_from_light.a = 1.0;
                    diffuse_color = diffuse_color.add(&color_from_light);	
                }
            }
        }

        // temporary, only use red channel of shininess for all colors
        let shiny_mult = min_shape.shininess.r;
        let trans_mult = (1.0 - shiny_mult) * min_shape.transparency.r;
        let diffuse_mult = 1.0 - shiny_mult - trans_mult;

        let weighted_diffuse = diffuse_color.scale(diffuse_mult);
        let weighted_shininess = shiny_color.scale(shiny_mult);
        let weighted_transparency = trans_color.scale(trans_mult);

        let mut final_color = weighted_diffuse.add(&weighted_shininess).add(&weighted_transparency);
        final_color.a = 1.0;

        println!("{}: -> {:?}", level, final_color);

        return final_color;
    }
}

fn mix(a: &f64, b: &f64, mix: &f64) -> f64 {
    return b * mix + a * (1.0 - mix); 
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }
    let filename = &args[1];
    let img = parse::parse(filename);

    let mut r = Raytracer::new(img.cfg.width, img.cfg.height, img.spheres, img.suns, img.bulbs, img.bounces);
    r.trace_from_camera();
    r.save(&img.cfg.filename);
}
