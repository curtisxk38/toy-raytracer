extern crate image;

use image::Rgba;

struct Raytracer {
    width: u32,
    height: u32,
    imgbuf: image::ImageBuffer<Rgba<u8>, Vec<u8>>
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
}

fn main() {
    let imgx = 800;
    let imgy = 800;
    let mut r = Raytracer::new(imgx, imgy);
    r.fill_black();
    r.save("test.png");
}
