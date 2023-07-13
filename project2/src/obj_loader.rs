use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use std::collections::HashMap;
use std::iter::Skip;
use std::str::Split;

use cgmath::{Vector3, Vector4};

pub struct ObjModel {
    pub vertices: Vec<Vector4<f32>>,
    pub vertex_normals: Vec<Vector3<f32>>,
    pub texture_coords: Vec<Vector3<f32>>,
    pub groups_vertices: HashMap<String, Vec<Vector4<f32>>>
}

impl ObjModel {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let mut vertices: Vec<Vector4<f32>> = Vec::new();
        let mut vertex_normals: Vec<Vector3<f32>> = Vec::new();
        let mut texture_coords: Vec<Vector3<f32>> = Vec::new();
        let mut groups_vertices: HashMap<String, Vec<Vector4<f32>>> = HashMap::new();

        // Open the file
        let file = File::open(path)?;
        let buf = BufReader::new(file);

        for (i, line_result) in buf.lines().enumerate() {
            let line_res = line_result.unwrap();
            let line = line_res.trim();

            // Skip empty lines
            if line.len() == 0 as usize {
                continue;
            }

            let mut parts = line.split(" ");
            let command = parts.nth(0).unwrap();
            match command {
                "#" => continue,
                "v" => {
                    vertices.push(Self::parse_vec4(parts.skip(1), Some(1.0)));
                }
                "vn" => {
                    vertex_normals.push(Self::parse_vec3(parts.skip(1), None));
                }
                "vt" => {
                    texture_coords.push(Self::parse_vec3(parts.skip(1), Some(0.0)));
                }
                "g" => {

                }
                _ => eprintln!(
                    "Warning: Skipping line {} with unknown command {}",
                    i, command
                ),
            }
        }

        Ok(Self { vertices, vertex_normals, texture_coords, groups_vertices })
    }

    fn parse_vec3(parts: Skip<Split<&str>>, last_value: Option<f32>) -> Vector3<f32> {
        let converted_parts = parts
            .map(|item| item.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();

        return Vector3 {
            x: converted_parts[0],
            y: converted_parts[1],
            z: if converted_parts.len() < 3 { last_value.unwrap_or(1.0) } else { converted_parts[2] },
        };
    }

    fn parse_vec4(parts: Skip<Split<&str>>, last_value: Option<f32>) -> Vector4<f32> {
        let converted_parts = parts
            .map(|item| item.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();

        return Vector4 {
            x: converted_parts[0],
            y: converted_parts[1],
            z: converted_parts[2],
            w: if converted_parts.len() < 4 { last_value.unwrap_or(1.0) } else { converted_parts[3] },
        };
    }
}
