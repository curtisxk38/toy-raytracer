
pub struct Bulb {
    pub position: Vector3,
    pub color: Color
}

// a sun light infinitely far away in the <direction> direction.
// The “direction to light” vector in the lighting equation is given direction
//   no matter where the object is.
pub struct Sun {
    pub direction: Vector3,
    pub color: Color
}

impl Sun {
    pub fn new(direction: Vector3, color: Color) -> Sun {
        Sun {direction: direction.normalize(), color }
    }
}

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

impl Color {
	pub fn white() -> Color {
		Color {r: 1.0, g: 1.0, b: 1.0, a: 1.0}
    }
    
    pub fn clone(&self) -> Color {
        return Color {r: self.r, g: self.g, b: self.b, a: self.a}
    }

    pub fn clamp_and_convert(&self, channel: f64) -> u8 {
        let min = 0;
        let max = 255;
        let channel = channel * 255.0;
        let channel = channel.round() as i64;
        let channel = if channel < min { min } else if channel > max { max } else { channel };
        channel as u8
    }
    pub fn to_bytes_color(&self) -> image::Rgba<u8> {
        let r = self.clamp_and_convert(self.r);
        let g = self.clamp_and_convert(self.g);
        let b = self.clamp_and_convert(self.b);
        let a = self.clamp_and_convert(self.a);
        image::Rgba::<u8>([r, g, b, a])
    }

    pub fn add(&self, other: &Color) -> Color {
        Color {r: self.r + other.r, g: self.g + other.g, b: self.b + other.b, a: self.a + other.a}
    }

    pub fn mul(&self, other: &Color) -> Color {
        Color {r: self.r * other.r, g: self.g * other.g, b: self.b * other.b, a: self.a * other.a}
    }

    pub fn scale(&self, f: f64) -> Color {
        Color {r: self.r * f, g: self.g * f, b: self.b * f, a: self.a * f}
    }
}

pub struct Sphere {
    pub center: Vector3,
    pub r: f64,
    pub color: Color,
    pub contains_camera: bool,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, color: Color) -> Sphere {
        Sphere { center: center, r: radius, color: color, contains_camera: false }
    }
    pub fn intersect(&self, ray: &Ray) -> f64 {
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

    pub fn normal(&self, point: &Vector3) -> Vector3 {
        // find the vector normal to this shape at given point
        let center_to_point = point.subtract(&self.center);
        center_to_point.normalize()
    }
}

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }

    pub fn subtract(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn scale(&self, f: f64) -> Vector3 {
        Vector3 {x: self.x * f, y: self.y * f, z: self.z * f}
    }

    pub fn magnitude(&self) -> f64 {
        return self.dot(&self).sqrt();
    }

    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        Vector3 { x: self.x / mag, y: self.y / mag, z: self.z / mag }
    }

    pub fn clone(&self) -> Vector3 {
        Vector3 {x: self.x, y: self.y, z: self.z}
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        let direction = direction.normalize();
        Ray { origin, direction }
    }
}

pub struct ImageConfig {
	pub width: u32,
	pub height: u32,
	pub filename: String

}