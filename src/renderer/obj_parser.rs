use crate::general::positions_3d::{IndicesTriangle, Mesh, Point as Point3, Triangle as Triangle3};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub struct ObjParser {
    normals: Vec<Point3>,
    mesh: Mesh,
}

enum LineParseResult {
    Error(String),
    Parsed,
    Skipped,
}

impl From<Result<(), String>> for LineParseResult {
    fn from(result: Result<(), String>) -> Self {
        match result {
            Ok(_) => LineParseResult::Parsed,
            Err(message) => LineParseResult::Error(message),
        }
    }
}

impl ObjParser {
    fn new() -> Self {
        ObjParser {
            normals: Vec::new(),
            mesh: Mesh::new(),
        }
    }
    fn add_vertex(&mut self, vertex: Point3) {
        self.mesh.points.push(vertex);
    }

    pub fn parse_file(file_path: &PathBuf) -> Result<Mesh, String> {
        if file_path.as_path().extension() != Some(OsStr::new("obj")) {
            return Err(String::from("file must have .obj extension"));
        }

        let mut obj_parser = ObjParser::new();
        let mut line_number = 0;
        let mut parsed_lines = 0;

        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                line_number += 1;
                if let Ok(line) = line {
                    match obj_parser.handle_line(&line) {
                        LineParseResult::Error(message) => {
                            return Err(ObjParser::error(message, line, line_number))
                        }
                        LineParseResult::Parsed => {
                            parsed_lines += 1;
                        }
                        LineParseResult::Skipped => (),
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

        if parsed_lines == 0 {
            return Err(String::from("did not find any obj data"));
        }
        if obj_parser.mesh.indices_triangles.len() == 0 {
            return Err(String::from("returned a mesh without any triangles"));
        }

        Ok(obj_parser.mesh)
    }

    fn handle_line(&mut self, line: &str) -> LineParseResult {
        let space_separated_strings: Vec<&str> =
            line.split(' ').filter(|str| !str.is_empty()).collect();
        if space_separated_strings.len() < 2 {
            return LineParseResult::Skipped;
        }
        let command_string = space_separated_strings[0];
        let argument_strings = &space_separated_strings[1..];

        match command_string {
            "v" => self.handle_v(argument_strings).into(),
            "vn" => self.handle_vn(argument_strings).into(),
            "f" => self.handle_f(argument_strings).into(),
            _ => LineParseResult::Skipped,
        }
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
            self.add_vertex(Point3::from_vec(argument_nums).unwrap());
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
            .map(|str| parse_face_element_vertex_string(str))
            .collect();

        let vertices_indices: Vec<usize> = parsed_numbers
            .iter()
            .map(|indices| indices[0].expect("vertex index in face declaration") - 1)
            .collect();

        let vertices: Vec<&Point3> = vertices_indices
            .iter()
            .map(|vertex_index| &self.mesh.points[*vertex_index])
            .collect();

        let vertext_normal_indices_option: Vec<Option<usize>> =
            parsed_numbers.iter().map(|indices| indices[2]).collect();

        let face_normal: Point3 = match evaluate(vertext_normal_indices_option) {
            Some(indices) => {
                let vertex_normals: Vec<&Point3> = indices
                    .iter()
                    .map(|index| &self.normals[index - 1])
                    .collect();

                if all_equal(&vertex_normals).unwrap() {
                    let cloned_normal: Point3 = vertex_normals[0].clone();
                    cloned_normal
                } else {
                    return Err("face normals are different".into());
                }
            }
            None => Triangle3::get_normal_ref(&vertices[0..3]),
        };

        // TODO support negative indices
        let mut triangle = IndicesTriangle {
            p1: vertices_indices[0],
            p2: vertices_indices[1],
            p3: vertices_indices[2],
            normal: face_normal.clone(),
        };
        triangle.make_clockwise(&self.mesh.points);
        self.mesh.indices_triangles.push(triangle);

        // source for order of verts: https://community.khronos.org/t/i-need-to-convert-quad-data-to-triangle-data/13269
        if vertices.len() == 4 {
            let mut triangle = IndicesTriangle {
                p1: vertices_indices[2],
                p2: vertices_indices[3],
                p3: vertices_indices[0],
                normal: face_normal,
            };
            triangle.make_clockwise(&self.mesh.points);
            self.mesh.indices_triangles.push(triangle);
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
fn parse_face_element_vertex_string(string: &str) -> [Option<usize>; 3] {
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

pub fn all_equal<T: PartialEq>(elements: &[T]) -> Option<bool> {
    let first = elements.get(0)?;
    Some(elements.iter().all(|elem| elem == first))
}

fn evaluate<T: Clone>(a: Vec<Option<T>>) -> Option<Vec<T>> {
    let mut new_vec: Vec<T> = Vec::new();
    for element in a {
        match element {
            None => return None,
            Some(value) => {
                let copy: T = value.clone();
                new_vec.push(copy);
            }
        }
    }
    Some(new_vec)
}
