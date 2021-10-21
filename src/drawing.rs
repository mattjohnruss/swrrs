use std::mem;
use geometry::Vec2;

// V5 - final version?
pub fn draw_line(mut v0: Vec2<u32>, mut v1: Vec2<u32>,
                 rgba: [u8; 4], imgbuf: &mut image::RgbaImage) {
    let mut steep = false;

    // if the line is steep, transpose it
    if (v0.x as i32 - v1.x as i32).abs() < (v0.y as i32 - v1.y as i32).abs() {
        mem::swap(&mut v0.x, &mut v0.y);
        mem::swap(&mut v1.x, &mut v1.y);
        steep = true;
    }
    
    // always draw right-to-left
    if v0.x > v1.x {
        mem::swap(&mut v0, &mut v1);
    }

    let d: Vec2<i32> = Vec2::from(v1) - Vec2::from(v0);

    let derror2 = 2 * d.y.abs();
    let mut error2: i32 = 0;
    let mut y = v0.y;

    for x in v0.x..(v1.x + 1) {
        if steep {
            imgbuf.put_pixel(y, x, image::Rgba(rgba));
            //println!("{}\n{}", y, x);
        }
        else {
            imgbuf.put_pixel(x, y, image::Rgba(rgba));
            //println!("{}\n{}", x, y);
        }

        error2 += derror2;
        if error2 > d.x {
            if v1.y > v0.y {
                y += 1;
            } else {
                y -= 1;
            };
            error2 -= 2*d.x;
        }
    }
}

pub fn draw_triangle(mut vertices: [Vec2<u32>; 3], _rgba: [u8; 4], imgbuf: &mut image::RgbaImage) {
    //let mut vertices = vertices;

    // reorder the vertices into ascending y-coord order
    if vertices[0].y > vertices[1].y {
        vertices.swap(0, 1);
    }
    if vertices[0].y > vertices[2].y {
        vertices.swap(0, 2);
    }
    if vertices[1].y > vertices[2].y {
        vertices.swap(1, 2);
    }

    let _total_height = vertices[2].y as i32 - vertices[0].y as i32;
    let _segment = vertices[1].y as i32 - vertices[0].y as i32 + 1;

    for _y in vertices[0].y..vertices[1].y {
        //let alpha = (y - t0.y)
    }

    draw_line(vertices[0], vertices[1], [0, 255, 0, 255], imgbuf);
    draw_line(vertices[1], vertices[2], [0, 255, 0, 255], imgbuf);
    draw_line(vertices[2], vertices[0], [255, 0, 0, 255], imgbuf);
}

#[allow(dead_code)]
mod old {
    use std::mem;

    // V1
    pub fn draw_line_v1(x0: u32, y0: u32, x1: u32, y1: u32,
                        rgba: [u8; 4], imgbuf: &mut image::RgbaImage) {
        let steps: usize = 101;
        let dt: f32 = 1.0/((steps - 1) as f32);

        for step in 0..steps {
            let t = step as f32 * dt;
            let x = (x0 as f32 * (1.0 - t) + x1 as f32 * t) as u32;
            let y = (y0 as f32 * (1.0 - t) + y1 as f32 * t) as u32;
            imgbuf.put_pixel(x, y, image::Rgba(rgba));
        }
    }

    // V2
    pub fn draw_line_v2(x0: u32, y0: u32, x1: u32, y1: u32,
                        rgba: [u8; 4], imgbuf: &mut image::RgbaImage) {
        for x in x0..x1 {
            let xdiff0 = x as f32 - x0 as f32;
            let xdiff1 = x1 as f32 - x0 as f32;
            let y0 = y0 as f32;
            let y1 = y1 as f32;
            let t = xdiff0/xdiff1;
            let y = (y0 * (1.0 - t) + y1 * t) as u32;
            imgbuf.put_pixel(x, y, image::Rgba(rgba));
        }
    }

    // V3
    pub fn draw_line_v3(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32,
                        rgba: [u8; 4], imgbuf: &mut image::RgbaImage) {
        let mut steep = false;

        // if the line is steep, transpose it
        if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            steep = true;
        }
        
        // always draw right-to-left
        if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        for x in x0..x1 + 1 {
            let xdiff0 = x as f32 - x0 as f32;
            let xdiff1 = x1 as f32 - x0 as f32;
            let y0 = y0 as f32;
            let y1 = y1 as f32;
            let t = xdiff0/xdiff1;
            let y = (y0 * (1.0 - t) + y1 * t) as u32;
            if steep {
                imgbuf.put_pixel(y, x, image::Rgba(rgba));
            }
            else {
                imgbuf.put_pixel(x, y, image::Rgba(rgba));
            }
        }
    }

    // V4
    pub fn draw_line_v4(x0: u32, y0: u32, x1: u32, y1: u32,
                        rgba: [u8; 4], imgbuf: &mut image::RgbaImage) {
        // make mut bindings since we may need to swap them in place
        let mut x0 = x0;
        let mut x1 = x1;
        let mut y0 = y0;
        let mut y1 = y1;

        let mut steep = false;

        // if the line is steep, transpose it
        if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            steep = true;
        }
        
        // always draw right-to-left
        if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;

        let derror = (dy as f32 / dx as f32).abs();
        let mut error: f32 = 0.0;
        let mut y = y0;

        for x in x0..x1 + 1 {
            if steep {
                imgbuf.put_pixel(y, x, image::Rgba(rgba));
            }
            else {
                imgbuf.put_pixel(x, y, image::Rgba(rgba));
            }

            error += derror;
            if error > 0.5 {
                if y1 > y0 {
                    y += 1;
                } else {
                    y -= 1;
                };
                error -= 1.0;
            }
        }
    }
}
