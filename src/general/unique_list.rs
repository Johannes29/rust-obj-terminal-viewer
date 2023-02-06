use std::cmp::Ordering;
use std::fmt::Debug;
use crate::general::positions_3d::Point as Point3;


/// items[indices_in_added_order[6 - 1]] gives you the value
/// that was added during the sixth call to add() since calling new()
#[derive(Debug)]
pub struct UniqueList<T> {
    // TODO items should not be pub
    pub items: Vec<T>,
    identifying_bytes_list: Vec<Vec<u8>>,
    indices_in_added_order: Vec<usize>,
}

#[derive(Debug)]
pub enum SearchResult {
    AddAt(usize),
    IsAt(usize),
}

impl SearchResult {
    pub fn index(&self) -> usize {
        match self {
            SearchResult::AddAt(index) => *index,
            SearchResult::IsAt(index) => *index,
        }
    }
}

impl<T> UniqueList<T>
where
    Vec<u8>: From<T>,
    T: Clone + Debug,
{
    pub fn new() -> Self {
        UniqueList {
            items: Vec::new(),
            identifying_bytes_list: Vec::new(),
            indices_in_added_order: Vec::new(),
        }
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    /// `add_index` 0 to get the first item that was added
    /// `add_index` 100 to get the 101:th item that was added
    pub fn get_item(&self, add_index: usize) -> &T {
        let items_index = self.indices_in_added_order[add_index];
        // dbg!(self, add_index);
        &self.items[items_index]
    }

    /// returns the index of the added item,
    /// or the index of the already existing copy of the item
    pub fn add_if_unique(&mut self, item: T) {
        let identifying_bytes: Vec<u8> = item.clone().into();
        dbg!(&item, &identifying_bytes, &self);
        let search_result = search_list(&self.identifying_bytes_list, &identifying_bytes);
        if let SearchResult::AddAt(inserting_index) = search_result {
            self.identifying_bytes_list
                .insert(inserting_index, identifying_bytes);
            self.items.insert(inserting_index, item);
            self.shift_indices(inserting_index);
        }
        self.indices_in_added_order.push(search_result.index());
    }

    fn shift_indices(&mut self, insertion_index: usize) {
        if insertion_index != self.indices_in_added_order.len() {
            self.indices_in_added_order = self.indices_in_added_order
                .iter()
                .map(|index| {
                    if index >= &insertion_index {
                        *index + 1
                    } else {
                        *index
                    }
                })
                .collect();
        }
    }
}

/// Returns the index where the item should be added so that the list remains sorted,
/// or the index where an identical copy of the item already exists. 
/// Assumes that the list is sorted.
/// 
/// Assumes that all elements after the inserted element would shift to the right when inserting the element.
pub fn search_list(list: &Vec<Vec<u8>>, item: &Vec<u8>) -> SearchResult {
    if list.len() == 0 {
        return SearchResult::AddAt(0);
    }

    let mut min_index = 0;
    let mut max_index = list.len() - 1;
    loop {
        let split_index = (min_index + max_index) / 2;
        let split_item = &list[split_index];
        if max_index - min_index >= 2 {
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

impl From<Point3> for Vec<u8> {
    fn from(point: Point3) -> Self {
        let mut bytes = Vec::new();
        bytes.append(&mut point.x.to_le_bytes().into());
        bytes.append(&mut point.y.to_le_bytes().into());
        bytes.append(&mut point.z.to_le_bytes().into());
        bytes
    }
}

#[test]
fn test_unique_list() {
    let mut unique_list: UniqueList<Point3> = UniqueList::new();
    let points = vec![
        Point3 { x: 0.5, y: -0.3, z: 31.2 },
        Point3 { x: 2.5, y: 0.8, z: 1.6 },
        Point3 { x: -1.5, y: -4.3, z: 11.2 },
        Point3 { x: 3.5, y: -0.1, z: 1.9 },
        Point3 { x: -0.5, y: 4.4, z: 9.2 },
        Point3 { x: -0.5, y: 4.4, z: 9.2 },
        Point3 { x: 5.5, y: -1.7, z: -11.2 },
        Point3 { x: 5.5, y: -1.7, z: -11.2 },
    ];
    let duplicate_count = 2;
    for point in points.clone() {
        unique_list.add_if_unique(point.clone());
    }
    dbg!("after adding", &unique_list);
    assert_eq!(unique_list.items.len(), points.len() - duplicate_count);
    for index in 0..points.len() {
        let point_from_unique_list = unique_list.get_item(index);
        let original_point = &points[index];
        assert_eq!(point_from_unique_list, original_point);
    }
}