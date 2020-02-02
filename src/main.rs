extern crate image;

use std::convert::TryFrom;


struct Raytracer {
    width: u32,
    height: u32,
    imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    spheres: Vec<Sphere>,
    img: Vec<f64>
}

impl Raytracer {
    fn new(width: u32, height: u32, spheres: Vec<Sphere>) -> Raytracer {
        let size = width * height;
        let size = usize::try_from(size).unwrap();
        Raytracer { width: width, height: height,
            imgbuf: image::ImageBuffer::new(width, height),
            spheres: spheres,
            img: vec![0.0; size]
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
                let color_bytes = match { color } {
                    Some(color) => color.to_bytes_color(),
                    None => image::Rgba::<u8>([0, 0, 0, 0])
                };
                let pixel = self.imgbuf.get_pixel_mut(i, j);
                *pixel = color_bytes;
            }
        }
    }

    fn shoot_ray(&mut self, ray: Ray, _level: u32) -> Option<&Color> {
        

        let mut min_dist = self.spheres[0].intersect(&ray);
        let mut min_shape = &self.spheres[0];

        for s in &self.spheres[1..] {
            let intersect_dist = s.intersect(&ray);
            if intersect_dist > 0.0 && (intersect_dist < min_dist || min_dist < 0.0) {
                min_dist = intersect_dist;
                min_shape = &s;
            }
        }

        if min_dist < 0.0 {
            return None;
        }

        // lambertian reflectance
        let collision_point = ray.origin.add(&ray.direction.scale(min_dist));
        let normal = min_shape.normal(collision_point);
        // min_shape.get_color() * sun.color * normal.dot(sun.direction);


        return Some(&min_shape.color);
    }
}

// a sun light infinitely far away in the <direction> direction.
// The “direction to light” vector in the lighting equation is given direction
//   no matter where the object is.
struct Sun {
    direction: Vector3,
    color: Color
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64
}

impl Color {
    fn clamp_and_convert(&self, channel: f64) -> u8 {
        let min = 0.0;
        let max = 1.0;
        let channel = if channel < min { min } else if channel > max { max } else { channel };
        let channel = channel * 255.0;
        let channel = channel.round();
        channel as u8
    }
    fn to_bytes_color(&self) -> image::Rgba<u8> {
        let r = self.clamp_and_convert(self.r);
        let g = self.clamp_and_convert(self.g);
        let b = self.clamp_and_convert(self.b);
        let a = self.clamp_and_convert(self.a);
        image::Rgba::<u8>([r, g, b, a])
    }
}

struct Sphere {
    center: Vector3,
    r: f64,
    color: Color
}

impl Sphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        // vector from ray origin to center of sphere
        let oc = self.center.subtract(&ray.origin);
        let oc_mag_squared = oc.dot(&oc);

        let ray_d_mag = ray.direction.magnitude();

        let inside = oc_mag_squared < self.r * self.r;
        // point along ray, where it comes closest to center
        // scalar projection of ray onto oc
        let t_center = oc.dot(&ray.direction) / ray_d_mag;

        if !inside && t_center < 0.0 {
            return -1.0; // no collision
        }
        

        // distance of closest approach
        let d = ray.origin.add(&ray.direction.scale(t_center)).subtract(&self.center);
        let d_mag_squared = d.dot(&d);

        let r2_d2_diff = self.r * self.r - d_mag_squared;

        if !inside && d_mag_squared > self.r*self.r {
            return -1.0 // no collision
        }

        let t_offset = r2_d2_diff.sqrt() / ray_d_mag;

        if inside {
            return t_center + t_offset;
        } else {
            return t_center - t_offset;
        }
    }

    fn normal(&self, point: Vector3) {
        // find the vector normal to this shape at given point
        let center_to_point = point.subtract(&self.center);
        center_to_point.normalize();
    }
}

#[derive(Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }

    fn subtract(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }

    fn mul(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }

    fn dot(&self, other: &Vector3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    fn scale(&self, f: f64) -> Vector3 {
        Vector3 {x: self.x * f, y: self.y * f, z: self.z * f}
    }

    fn magnitude(&self) -> f64 {
        return self.dot(&self).sqrt();
    }

    fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        Vector3 { x: self.x / mag, y: self.y / mag, z: self.z / mag }
    }
}

#[derive(Debug)]
struct Ray {
    origin: Vector3,
    direction: Vector3,
}

fn save_image(raytracer: &Raytracer) {

}

fn main() {
    let imgx = 100;
    let imgy = 50;
    let s1 = Sphere {
        center: Vector3{x: 0.0, y: 0.0, z: -1.0},
        r: 0.3,
        color: Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0}
    };
    let s2 = Sphere {
        center: Vector3{x: 1.0, y: 0.8, z: -1.0},
        r: 0.5,
        color: Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0}
    };
    let shapes = vec![s1, s2];

    let mut r = Raytracer::new(imgx, imgy, shapes);
    r.trace_from_camera();
    r.save("test.png");
}
