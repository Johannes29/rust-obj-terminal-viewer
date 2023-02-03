use crate::general::positions_3d::{Mesh, Point as Point3, Triangle as Triangle3, IndicesTriangle};
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub struct ObjParser {
    unique_vertices: UniqueList<Point3>,
    vertices_indices: Vec<usize>,
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
            unique_vertices: UniqueList::new(),
            vertices_indices: Vec::new(),
            normals: Vec::new(),
            mesh: Mesh::new(),
        }
    }

    /// Returns the index of unique_vertices where the vertex with specified obj reference number can be found
    fn get_unique_verts_index(&self, vertex_reference_number: usize) -> usize {
        self.vertices_indices[vertex_reference_number - 1]
    }

    /*
    fn get_vertex(&self, vertex_reference_number: usize) -> Box<Point3> {
        // Vertex reference numbers start at 1
        let unique_verts_index = self.get_unique_verts_index(vertex_reference_number);
        Box::from(self.unique_vertices.items[unique_verts_index])
    } */

    fn add_vertex(&mut self, vertex: Point3) {
        // TODO this code is commented out because unique_vertices.add() seems to get stuck
        // let index = self.unique_vertices.add(vertex);
        // self.vertices_indices.push(index);
        self.unique_vertices.items.push(vertex);
        self.vertices_indices.push(self.vertices_indices.len());
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
            .map(|str| parse_face_element_vertext_string(str))
            .collect();

        let vertices_indices: Vec<usize> = parsed_numbers
            .iter()
            .map(|indices| indices[0].expect("vertex index in face declaration"))
            .map(|vertex_reference_number| self.get_unique_verts_index(vertex_reference_number))
            .collect();

        let vertices: Vec<&Point3> =vertices_indices
            .iter()
            .map(|unique_verts_index| &self.unique_vertices.items[*unique_verts_index])
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
        triangle.make_clockwise(&self.mesh.points).unwrap();
        self.mesh.indices_triangles.push(triangle);

        // source for order of verts: https://community.khronos.org/t/i-need-to-convert-quad-data-to-triangle-data/13269
        if vertices.len() == 4 {
            let mut triangle = IndicesTriangle {
                p1: vertices_indices[2],
                p2: vertices_indices[3],
                p3: vertices_indices[0],
                normal: face_normal,
            };
            triangle.make_clockwise(&self.mesh.points).unwrap();
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

struct UniqueList<T> {
    items: Vec<T>,
    identifying_bytes_list: Vec<Vec<u8>>,
}

impl<T> UniqueList<T>
where
    Vec<u8>: From<T>,
    T: Clone,
{
    pub fn new() -> Self {
        UniqueList {
            items: Vec::new(),
            identifying_bytes_list: Vec::new(),
        }
    }


    /// returns the index of the added item,
    /// or the index of the already existing copy of the item
    pub fn add(&mut self, item: T) -> usize {
        let identifying_bytes: Vec<u8> = item.clone().into();
        let inserting_index = 
            if self.items.len() > 0 {
                match search_list(&self.identifying_bytes_list, &identifying_bytes) {
                    SearchResult::IsAt(index) => return index,
                    SearchResult::AddAt(index) => index,
                }
            } else { 0 };
        self.identifying_bytes_list
            .insert(inserting_index, identifying_bytes);
        self.items.insert(inserting_index, item);
        inserting_index
    }
}

/// Returns the index where the item should be added so that the list remains sorted,
/// or the index where an identical copy of the item already exists. 
/// Assumes that the list is sorted.
/// 
/// Assumes that all elements after the inserted element would shift to the right when inserting the element.
pub fn search_list(list: &Vec<Vec<u8>>, item: &Vec<u8>) -> SearchResult {
    let split_index = list.len() / 2;
    let split_item = &list[split_index];

    let mut min_index = 0;
    let mut max_index = list.len() - 1;
    let split_index = min_index + max_index / 2;
    loop {
        if max_index - min_index >= 2 {
            // TODO seems to get stuck here...
            match split_item.cmp(item) {
                Ordering::Equal => return SearchResult::IsAt(split_index),
                Ordering::Greater => {
                    max_index = split_index - 1;
                }
                Ordering::Less => {
                    min_index = split_index + 1;
                }
            }
        } else {
            let item1 = &list[min_index];
            let item2 = &list[max_index];
            if item == item1 {
                return SearchResult::IsAt(min_index);
            } else if item == item2 {
                return SearchResult::IsAt(max_index);
            }
            if item < item1 {
                return SearchResult::AddAt(min_index);
            }
            if item < item2 {
                return SearchResult::AddAt(max_index);
            }
            return SearchResult::AddAt(max_index + 1);
        }
    }
}

pub enum SearchResult {
    AddAt(usize),
    IsAt(usize),
}

#[test]
fn test_binary_search_index_to_add() {
    let item1 = vec![12, 120, 240];
    let item2 = vec![13, 130, 230];
    let item_not_in_list = vec![14, 140, 255];
    let list: Vec<Vec<u8>> = vec![item1.clone(), item2.clone()];
    // assert!(index_to_add(&list, &item1).is_none());
    // assert!(index_to_add(&list, &item2).is_none());
    // assert!(index_to_add(&list, &item_not_in_list).is_some());
}

impl From<Point3> for Vec<u8> {
    fn from(point: Point3) -> Self {
        let mut bytes = Vec::new();
        bytes.append(&mut point.x.to_le_bytes().into());
        bytes.append(&mut point.y.to_le_bytes().into());
        bytes.append(&mut point.z.to_le_bytes().into());
        bytes
    }
}
