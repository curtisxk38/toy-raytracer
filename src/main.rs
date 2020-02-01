

extern crate image;


struct Raytracer {
    width: u32,
    height: u32,
    imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    spheres: Vec<Sphere>
}

impl Raytracer {
    fn new(width: u32, height: u32, spheres: Vec<Sphere>) -> Raytracer {
        Raytracer { width: width, height: height,
            imgbuf: image::ImageBuffer::new(width, height),
            spheres: spheres
        }
    }

    fn save(self, filename: &str) {
        self.imgbuf.save(filename).unwrap();
    }

    fn get_ray_from_pixel(&mut self, x: f64, y: f64) -> Ray {
        let EYE = Vector3 {x: 0.0, y: 0.0, z: 0.0};
        let FORWARD = Vector3 {x: 0.0, y: 0.0, z: -1.0};
        let RIGHT = Vector3 {x: 1.0, y: 0.0, z: 0.0};
        let UP = Vector3 {x: 0.0, y: 1.0, z: 0.0};

        let max_dim = x.max(y);
        let max_dim: f64 = f64::from(max_dim);
        let sx = (2.0 * x - f64::from(self.width)) / max_dim;
        let sy = (f64::from(self.height) - 2.0 * y) / max_dim;

        let dir = FORWARD.add(&RIGHT.scale(sx)).add(&UP.scale(sy));

        return Ray {origin: EYE, direction: dir};

    }

    fn trace_from_camera(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let ray = self.get_ray_from_pixel(f64::from(i), f64::from(j));
                let color = self.shoot_ray(ray, 0);
                let pixel = self.imgbuf.get_pixel_mut(i, j);
                *pixel = color;
            }
        }
    }

    fn shoot_ray(&mut self, ray: Ray, _level: u32) -> image::Rgba<u8> {
        let color = image::Rgba([0, 0, 0, 0]);

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
            return color;
        }


        return min_shape.color;
    }
}

struct Sphere {
    center: Vector3,
    r: f64,
    color: image::Rgba<u8>
}

impl Sphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        // vector from ray origin to center of sphere
        let oc = ray.origin.subtract(&self.center);
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

        if !inside && r2_d2_diff < 0.0 {
            return -1.0 // no collision
        }

        let t_offset = r2_d2_diff.sqrt() / ray_d_mag;

        if inside {
            t_center + t_offset
        } else {
            t_center - t_offset
        }
    }
}

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

    fn dot(&self, other: &Vector3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    fn scale(&self, f: f64) -> Vector3 {
        Vector3 {x: self.x * f, y: self.y * f, z: self.z * f}
    }

    fn magnitude(&self) -> f64 {
        return self.dot(&self).sqrt();
    }
}

struct Ray {
    origin: Vector3,
    direction: Vector3,
}

fn main() {
    let imgx = 800;
    let imgy = 800;
    //sphere 0 0 -1 0.3
    //sphere 1 0.8 -1 0.5
    let s1 = Sphere {
        center: Vector3{x: 0.0, y: 0.0, z: -1.0},
        r: 0.3,
        color: image::Rgba([0, 0, 0, 255])
    };
    let shapes = vec![s1];

    let mut r = Raytracer::new(imgx, imgy, shapes);
    r.trace_from_camera();
    r.save("test.png");
}
