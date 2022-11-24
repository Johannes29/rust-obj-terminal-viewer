use crossterm::{cursor, QueueableCommand};
use std::io::{Write, stdout};

// TODO compare performance between replacing only the changed characters and printing out everything again

pub fn draw_char_buffer(char_buffer: &Vec<Vec<u8>>, prev_char_buffer: &Vec<Vec<u8>>) {
    let mut stdout = stdout();

    if char_buffer.is_empty() || prev_char_buffer.is_empty() || char_buffer[0].is_empty() || prev_char_buffer[0].is_empty()
        || char_buffer.len() != prev_char_buffer.len() || char_buffer[0].len() != prev_char_buffer[0].len() {
        panic!("pixel vectors have different sizes");
    }
    let mut chars_to_change: Vec<(u8, usize, usize)> = Vec::new();
    for (row_i, row) in char_buffer.iter().enumerate() {
        for (char_i, char) in row.iter().enumerate() {
            let prev_char = prev_char_buffer[row_i][char_i];

            if char != &prev_char {
                chars_to_change.push((*char, row_i, char_i))
            }
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

fn char_buffer_to_output_string(char_buffer: &mut Vec<Vec<u8>>) {
    let mut output_string = String::new();
    
    for row in char_buffer {
        for char in row {
            output_string.push((*char).into());
        }
    }
}

pub fn image_buffer_to_char_buffer(image_buffer: &Vec<Vec<f32>>, char_buffer: &mut Vec<Vec<u8>>, chars: &Vec<u8>) {
    // TODO #anti_aliasing: scale image_buffer to fit char_buffer in *separate function*
    if image_buffer.len() != char_buffer.len() || image_buffer[0].len() != char_buffer[0].len() {
        panic!("image_buffer and char_buffer have different shapes")
    }

    for i in 0..(image_buffer.len()) {
        for j in 0..(image_buffer[0].len()) {
            let value = image_buffer[i][j];
            let char_index = ((value * (chars.len() as f32) - 1.0)
                .ceil() as usize)
                .clamp(0, chars.len() - 1);

            char_buffer[i][j] = chars[char_index];
        }
    }
}