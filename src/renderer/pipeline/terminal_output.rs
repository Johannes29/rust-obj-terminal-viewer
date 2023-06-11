use crossterm::{cursor, QueueableCommand};
use std::io::{Write, stdout};

use crate::renderer::interface::Buffer;

pub fn draw_char_buffer(char_buffer: &Buffer<u8>, prev_char_buffer: &Buffer<u8>) {
    let mut stdout = stdout();
    let mut chars_to_change: Vec<(u8, usize, usize)> = Vec::new();

    for (i, char) in char_buffer.values.iter().enumerate() {
        let prev_char = prev_char_buffer.values[i];
        let x = i % char_buffer.width;
        let y = i / char_buffer.width;
        if char != &prev_char {
            chars_to_change.push((*char, y, x))
        }
    }

    stdout.queue(cursor::SavePosition).unwrap();

    for (new_char, row_i, char_i) in chars_to_change {
        // let char_number = new_char.to_digit(10).unwrap() as u8;

        stdout.queue(cursor::MoveTo(char_i as u16 + 1, row_i as u16)).unwrap();
        stdout.write(&[0x08, new_char]).unwrap();
    }

    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();
}

// TODO compare performance between replacing only the changed characters and printing out everything again
#[allow(dead_code)]
fn char_buffer_to_output_string(char_buffer: &mut Vec<Vec<u8>>) {
    let mut output_string = String::new();
    
    for row in char_buffer {
        for char in row {
            output_string.push((*char).into());
        }
    }
}

pub fn image_buffer_to_char_buffer(image_buffer: &Buffer<f32>, char_buffer: &mut Buffer<u8>, chars: &Vec<u8>) {
    for y in 0..(image_buffer.height) {
        for x in 0..(image_buffer.width) {
            let value = image_buffer.get(x, y).expect("x and y loops to be correct");
            let char_index = ((value * (chars.len() as f32) - 1.0)
                .ceil() as usize)
                .clamp(0, chars.len() - 1);

            char_buffer.set(x, y, chars[char_index]).unwrap();
        }
    }
}

pub fn add_debug_line_to_char_buffer(char_buffer: &mut Buffer<u8>, line: &str) {
    let mut chars: Vec<u8> = line.as_bytes().to_vec();
    chars.resize(char_buffer.width, b' ');

    let y = char_buffer.height - 1;
    for x in 0..(char_buffer.width) {
        char_buffer.set(x, y, chars[x]).unwrap();
    }
}