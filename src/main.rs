use std::time::Instant;

mod model;
mod geometry;
mod drawing;

use geometry::Vec2;

fn triangles() {
    let size_x = 200;
    let size_y = 200;

    let mut imgbuf = image::RgbaImage::new(size_x, size_y);

    let now = Instant::now();

    let t0 = [Vec2 {x: 10, y: 70}, Vec2 {x: 50, y: 160}, Vec2 {x: 70, y: 80}];
    let t1 = [Vec2 {x: 180, y: 50}, Vec2 {x: 150, y: 1}, Vec2 {x: 70, y: 180}];
    let t2 = [Vec2 {x: 180, y: 150}, Vec2 {x: 120, y: 160}, Vec2 {x: 130, y: 180}];

    drawing::draw_triangle(t0, [255, 0, 0, 255], &mut imgbuf);
    drawing::draw_triangle(t1, [255, 255, 255, 255], &mut imgbuf);
    drawing::draw_triangle(t2, [0, 255, 0, 255], &mut imgbuf);

    let duration = now.elapsed();

    println!("{}s + {}micros", duration.as_secs(), duration.subsec_micros());

    imgbuf = image::imageops::flip_vertical(&imgbuf);
    imgbuf.save("triangles.png").unwrap();
}

fn head() {
    let size_x = 800;
    let size_y = 800;

    let mut imgbuf = image::RgbaImage::new(size_x, size_y);

    let now = Instant::now();

    let african_head = model::Model::from_wavefront_file("african_head.obj").unwrap();
    african_head.draw_on_image(&mut imgbuf, size_x, size_y);

    let duration = now.elapsed();

    println!("{}s + {}micros", duration.as_secs(), duration.subsec_micros());

    imgbuf = image::imageops::flip_vertical(&imgbuf);
    imgbuf.save("model.png").unwrap();
}

fn main() {
    triangles();
    head();
}
