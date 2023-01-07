use crate::general::positions_3d::{Mesh, Point as Point3, Triangle as Triangle3};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub struct ObjParser {
    points: Vec<Point3>,
    normals: Vec<Point3>,
    mesh: Mesh,
}

impl ObjParser {
    fn new() -> Self {
        ObjParser {
            points: Vec::new(),
            normals: Vec::new(),
            mesh: Mesh::new(),
        }
    }

    pub fn parse_file(file_path: &PathBuf) -> Result<Mesh, String> {
        let mut obj_parser = ObjParser::new();
        let mut line_number = 0;

        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                line_number += 1;
                if let Ok(line) = line {
                    match obj_parser.handle_line(&line) {
                        Err(message) => return Err(ObjParser::error(message, line, line_number)),
                        Ok(_) => (),
                    };
                } else {
                    return Err(String::from("could not read line"));
                }
            }
        } else {
            return Err(format!(
                "Could not read file '{}'",
                file_path.to_str().expect("valid unicode")
            ));
        }

        Ok(obj_parser.mesh)
    }

    fn handle_line(&mut self, line: &str) -> Result<(), String> {
        let space_separated_strings: Vec<&str> =
            line.split(' ').filter(|str| !str.is_empty()).collect();
        if space_separated_strings.len() < 2 {
            return Ok(());
        }
        let command_string = space_separated_strings[0];
        let argument_strings = &space_separated_strings[1..];
        return match command_string {
            "v" => self.handle_v(argument_strings),
            "vn" => self.handle_vn(argument_strings),
            "f" => self.handle_f(argument_strings),
            _ => Ok(()),
        };
    }

    fn error(message: String, line: String, line_number: usize) -> String {
        return format!("{message}\nAt line {line_number}: '{line}'");
    }

    fn handle_v(&mut self, argument_strings: &[&str]) -> Result<(), String> {
        if argument_strings.len() != 3 {
            return Err("invalid amount of coordinate components (should be 3)".into());
        }

        let argument_nums: Vec<f32> = argument_strings
            .iter()
            .filter_map(|str| str.parse().ok())
            .collect();

        if argument_nums.len() == argument_strings.len() {
            self.points.push(Point3::from_vec(argument_nums).unwrap());
            return Ok(());
        } else {
            return Err("error when parsing verts".into());
        }
    }

    fn handle_vn(&mut self, argument_strings: &[&str]) -> Result<(), String> {
        if argument_strings.len() != 3 {
            return Err(String::from(
                "invalid amount of vertex normal components (should be 3)",
            ));
        }

        let argument_nums: Vec<f32> = argument_strings
            .iter()
            .filter_map(|str| str.parse().ok())
            .collect();
        if argument_nums.len() != argument_strings.len() {
            return Err(String::from("error when parsing vertex normal vector"));
        }
        self.normals.push(Point3::from_vec(argument_nums).unwrap());
        Ok(())
    }

    fn handle_f(&mut self, argument_strings: &[&str]) -> Result<(), String> {
        if argument_strings.len() != 3 && argument_strings.len() != 4 {
            return Err(String::from(
                "invalid number of verts per face (should be 3 or 4)",
            ));
        }

        let parsed_numbers: Vec<[Option<usize>; 3]> = argument_strings
            .iter()
            .map(|str| parse_face_element_vertext_string(str))
            .collect();

        let vertex_position_indices: Vec<usize> = parsed_numbers
            .iter()
            .map(|indices| indices[0].unwrap()) // TODO add error on unwrap
            .collect();

        let normals_are_provided = parsed_numbers.iter().all(|numbers| !numbers[2].is_none());
        if !normals_are_provided {
            return Err(String::from("no normals provided"));
        }

        let vertex_normal_indices: Vec<usize> = parsed_numbers
            .iter()
            .map(|indices| indices[2].unwrap())
            .collect();

        let vertex_normals: Vec<&Point3> = vertex_normal_indices
            .clone()
            .iter()
            .map(|index| &self.normals[index - 1])
            .collect();

        // check if all normals for this face are identical
        // let mut first_normal: Option<&Point3> = None;
        // let mut all_normals_are_identical = true;
        // for vertex_normal in &vertex_normals {
        //     if let None = first_normal {
        //         first_normal = Some(vertex_normal);
        //     } else {
        //         if Some(*vertex_normal) != first_normal {
        //             all_normals_are_identical = false
        //         }
        //     }
        // }
        // if !all_normals_are_identical {
        //     return Err(error("face normals are different".into(), line, line_number));
        // }
        let face_normal = self.normals[vertex_normal_indices[0] - 1].clone();

        // TODO support negative indices
        // TODO cloning is not optimal for performance
        let triangle = Triangle3::from_arr_n([
            self.points[vertex_position_indices[0] - 1].clone(),
            self.points[vertex_position_indices[1] - 1].clone(),
            self.points[vertex_position_indices[2] - 1].clone(),
            face_normal.clone(),
        ]);
        self.mesh.triangles.push(triangle);

        // source for order of verts: https://community.khronos.org/t/i-need-to-convert-quad-data-to-triangle-data/13269
        if vertex_position_indices.len() == 4 {
            let triangle = Triangle3::from_arr_n([
                self.points[vertex_position_indices[2] - 1].clone(),
                self.points[vertex_position_indices[3] - 1].clone(),
                self.points[vertex_position_indices[0] - 1].clone(),
                face_normal.clone(),
            ]);
            self.mesh.triangles.push(triangle);
        }

        Ok(())
    }
}

fn read_lines(file_path: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

/**
 * example inputs: "3/4/5", "3", "3//4", "3//"
 */
fn parse_face_element_vertext_string(string: &str) -> [Option<usize>; 3] {
    let substrings: Vec<&str> = string.split('/').collect();
    let mut numbers = [None, None, None];

    for i in 0..=2 {
        let substring = substrings.get(i);
        if let Some(substring) = substring {
            numbers[i] = substring.parse().ok();
        }
    }

    numbers
}
