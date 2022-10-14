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

                        let vert_texture_normal_indexes: Vec<[Option<usize>; 3]> = argument_strings
                            .iter()
                            .map(|str| parse_face_element_vertext_string(str))
                            .collect();

                        let vertex_indices: Vec<usize> = vert_texture_normal_indexes
                            .iter()
                            .map(|[ver_index, _, _]| ver_index)
                            .map(|index| index.unwrap())
                            .collect();

                        let normal_indices: Vec<usize> = vert_texture_normal_indexes
                            .iter()
                            .map(|[_, _, normal_index]| normal_index)
                            .map(|index| index.unwrap())
                            .collect();
                        

                        let vertex_normals: Vec<&Point3> = normal_indices
                            .clone()
                            .iter()
                            .map(|index| &normals[index - 1])
                            .collect();


                        // check if all normals for this face are identical
                        let mut first_normal: Option<Point3> = None;
                        let mut all_normals_are_identical = true;
                        for normal_index in &normal_indices {
                            let this_normal = normals[normal_index - 1].clone();
                            if let None = first_normal {
                                first_normal = Some(this_normal);
                            } else {
                                if Some(this_normal) != first_normal {
                                    all_normals_are_identical = false
                                }
                            }
                        }
                        if !all_normals_are_identical {
                            panic!("face normals are different");
                        }
                        let face_normal = normals[normal_indices[0] - 1].clone();

                        if vertex_indices.len() == argument_strings.len() {
                            // TODO support negative indices
                            // TODO cloning is not optimal for performance
                            let triangle = Triangle3::from_arr_n([
                                points[vertex_indices[0] - 1].clone(),
                                points[vertex_indices[1] - 1].clone(),
                                points[vertex_indices[2] - 1].clone(),
                                face_normal.clone(),
                            ]);
                            mesh.triangles.push(triangle);

                            // source for order of verts: https://community.khronos.org/t/i-need-to-convert-quad-data-to-triangle-data/13269
                            if vertex_indices.len() == 4 {
                                let triangle = Triangle3::from_arr_n([
                                    points[vertex_indices[2] - 1].clone(),
                                    points[vertex_indices[3] - 1].clone(),
                                    points[vertex_indices[0] - 1].clone(),
                                    face_normal.clone(),
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

/**
 * example inputs: "3/4/5", "3", "3//4", "3//"
 */
fn parse_face_element_vertext_string(string: &str) -> [Option<usize>; 3] {
    let substrings: Vec<&str> = string.split('/').collect();
    let mut numbers  = [None, None, None];

    for i in 0..=2 {
        let substring = substrings.get(i);
        if let Some(substring) = substring {
            numbers[i] = substring.parse().ok();
        }
    }

    numbers
}