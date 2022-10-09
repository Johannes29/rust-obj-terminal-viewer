use crate::general::positions_3d::{Mesh, Triangle as Triangle3, Point as Point3};
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_obj(file_path: &str) -> Mesh {
    let mut points = Vec::new();
    let mut normals = Vec::new();
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
                    "vn" => {
                        match argument_strings.len() {
                            3 => {
                                let argument_nums: Vec<f32> = argument_strings.iter()
                                .filter_map(|str| str.parse().ok())
                                .collect();

                                if argument_nums.len() == argument_strings.len() {
                                    normals.push(Point3::from_vec(argument_nums).unwrap());
                                } else {
                                    panic!("error when parsing vertex normal vector")
                                }
                            },
                            _ => panic!("invalid number of normal vertex components"),
                        }
                    },
                    "f" => {
                        if argument_strings.len() > 4 {
                            panic!("too many verts per face");
                        }
                        if argument_strings.len() < 3 {
                            panic!("too few verts per face");
                        }

                        let point_indices: Vec<usize> = argument_strings.iter()
                            .map(|str| str.split('/').collect::<Vec<&str>>()[0])
                            .filter_map(|str| str.parse().ok())
                            .collect();

                        // TODO call parse_face_element_vertext_string() here

                        if point_indices.len() == argument_strings.len() {
                            // TODO support negative indices
                            // TODO cloning is not optimal for performance
                            let triangle = Triangle3::from_arr_n([
                                points[point_indices[0] - 1].clone(),
                                points[point_indices[1] - 1].clone(),
                                points[point_indices[2] - 1].clone(),
                                TODO_normal,
                            ]);
                            mesh.triangles.push(triangle);

                            // source for order of verts: https://community.khronos.org/t/i-need-to-convert-quad-data-to-triangle-data/13269
                            if point_indices.len() == 4 {
                                let triangle = Triangle3::from_arr_n([
                                    points[point_indices[2] - 1].clone(),
                                    points[point_indices[3] - 1].clone(),
                                    points[point_indices[0] - 1].clone(),
                                    TODO_normal,
                                ]);
                                mesh.triangles.push(triangle);
                            }
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

fn parse_face_element_vertext_string(string: &str) -> [Option<f32>; 3] {
    let substrings: Vec<&str> = string.split('/').collect();
    let mut numbers  = [None, None, None];

    for i in 0..2 {
        let substring = substrings.get(i);
        if let Some(substring) = substring {
            numbers[i] = substring.parse().ok();
        }
    }

    numbers
}