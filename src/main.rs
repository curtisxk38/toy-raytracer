extern crate image;

fn main() {
    let imgx = 800;
    let imgy = 800;
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }

    imgbuf.save("test.png").unwrap();
}
