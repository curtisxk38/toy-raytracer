

extern crate image;


struct Raytracer {
    width: u32,
    height: u32,
    imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
}

impl Raytracer {
    fn new(width: u32, height: u32) -> Raytracer {
        Raytracer { width: width, height: height, imgbuf: image::ImageBuffer::new(width, height) }
    }

    fn fill_black(&mut self) {
        for (_x, _y, pixel) in self.imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
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

    fn shoot_ray(&mut self, ray: Ray, level: u32) -> image::Rgba<u8> {
        let color = image::Rgba([0, 0, 0, 0]);
        return color;
    }
}

struct Sphere {
    center: Vector3,
    r: f64,
    color: image::Rgba<u8>
}

impl Sphere {
    fn intersect(self, ray: Ray) -> f64 {
        let oc = ray.origin.subtract(&self.center);
        let x = ray.direction.dot(&oc);
        let rad = x * x - oc.dot(&oc) + self.r * self.r;
        if rad < 0.0 {
            return -1.0;
        }
        let result = x * -1.0 - rad.sqrt();
        return result;
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
}

struct Ray {
    origin: Vector3,
    direction: Vector3,
}

fn main() {
    let imgx = 800;
    let imgy = 800;
    let mut r = Raytracer::new(imgx, imgy);
    r.fill_black();
    r.trace_from_camera();
    r.save("test.png");
}
