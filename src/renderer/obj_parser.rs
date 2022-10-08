use crate::general::positions_3d::{Mesh, Triangle as Triangle3, Point as Point3};
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_obj(file_path: &str) -> Mesh {
    let mut points = Vec::new();
    let mut mesh = Mesh { triangles: Vec::new() };

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                let space_separated_strings: Vec<&str> = line.split(' ').collect();
                let command_string = space_separated_strings[0];
                let argument_strings = &space_separated_strings[1..];
                match command_string {
                    "v" => {
                        if argument_strings.len() > 3 {
                            panic!("too many coordinate components (dimensions)");
                        }

                        let argument_nums: Vec<f32> = argument_strings.iter()
                            .filter_map(|str| str.parse().ok())
                            .collect();
    
                        if argument_nums.len() == argument_strings.len() {
                            points.push(Point3::from_vec(argument_nums).unwrap());
                        } else {
                            panic!("error when parsing verts")
                        }
                    },
                    "f" => {
                        if argument_strings.len() > 3 {
                            panic!("too many verts per face");
                        }

                        let point_indices: Vec<usize> = argument_strings.iter()
                            .map(|str| str.split('/').collect::<Vec<&str>>()[0])
                            .filter_map(|str| str.parse().ok())
                            .collect();

    
                        if point_indices.len() == argument_strings.len() {
                            // TODO support negative indices
                            // TODO cloning is not optimal for performance
                            let triangle = Triangle3::from_arr([
                                points[point_indices[0] - 1].clone(),
                                points[point_indices[1] - 1].clone(),
                                points[point_indices[2] - 1].clone(),
                            ]);
                            mesh.triangles.push(triangle);
                        } else {
                            panic!("error when parsing faces")
                        }
                    },
                    _ => (),
                }
            }
        }
    } else {
        panic!("Could not read file '{}'", file_path)
    }

    mesh
}

fn read_lines(file_path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}