use std::fs;
use std::io::Error;

use geometry::{Vec2, Vec3};
use drawing::draw_line;

type Vertex = Vec3<f32>;

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<[usize; 3]>,
}

impl Model {
    pub fn from_wavefront_file(filename: &str) -> Result<Model, Error> {
        let data = fs::read_to_string(filename)?;
        Model::from_wavefront_string(&data)
    }

    fn from_wavefront_string(data: &str) -> Result<Model, Error> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<[usize; 3]> = Vec::new();

        for line in data.lines() {
            //assume ASCII text only and use slices as characters
            let mut words = line.split_whitespace();

            match words.next() {
                Some("v") => {
                    let vertex: Vec<f32> = words.map(|w| w.parse().unwrap()).collect();
                    vertices.push(Vertex {
                        x: vertex[0],
                        y: vertex[1],
                        z: vertex[2]
                    });
                }
                Some("f") => {
                    let face: Vec<usize> = words
                        .map(|w| w.split("/").next().unwrap().parse::<usize>().unwrap() - 1)
                        .collect();
                    faces.push([face[0], face[1], face[2]]);
                }
                _ => ()
            }
        }

        Ok(Model {
            vertices,
            faces,
        })
    }

    pub fn draw_on_image(&self, imgbuf: &mut image::RgbaImage, width: u32, height: u32) {
        for face in &self.faces {
            for j in 0..3 {
                let v0 = &self.vertices[face[j]];
                let v1 = &self.vertices[face[(j + 1)%3]];

                // this assumes x, y are in the range [-1, 1] and the resulting line
                // endpoints are in the range [0, 1]
                let x0 = ((v0.x + 1.0)*(width as f32 - 1.0)/2.0) as u32;
                let y0 = ((v0.y + 1.0)*(height as f32 - 1.0)/2.0) as u32;
                let x1 = ((v1.x + 1.0)*(width as f32 - 1.0)/2.0) as u32;
                let y1 = ((v1.y + 1.0)*(height as f32 - 1.0)/2.0) as u32;
                
                let colour = [255, 255, 255, 255];

                draw_line(Vec2{x: x0, y: y0}, Vec2{x: x1, y: y1}, colour, imgbuf);
            }
        }
    }
}
