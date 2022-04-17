use super::consts::*;
use super::cursor;
use std::io::{stdout, stdin, Read, Write};
use failure::Error;
use super::error::TerminalError;

pub fn use_alternate_screen_buffer() {
    print!("{}{}", CSI, ALTERNATE_SCREEN_BUFFER);
}

pub fn use_main_screen_buffer() {
    print!("{}{}", CSI, MAIN_SCREEN_BUFFER);
}

fn read_until(ch: char) -> String {
    let mut accum = vec![];
    for rbyte in stdin().bytes() {
        let byte = rbyte.unwrap();
        accum.push(byte);

        if ch as u8 == byte {
            break;
        }
    }

    String::from_utf8(accum).unwrap()
}

pub fn dec_line_drawing() {
    print!("{}{}", ESC, DEC_LINE_DRAWING);
}

pub fn ascii_chars_drawing() {
    print!("{}{}", ESC, ASCII_CHARS_DRAWING);
}

pub fn get_screen_dimensions() -> Result<(usize, usize), Error> {
    cursor::move_to(999, 999);
    query_cursor_pos()
}

pub fn query_cursor_pos() -> Result<(usize, usize), Error> {
    print!("{}{}", CSI, QUERY_CURSOR_POS);
    stdout().flush().unwrap();

    let r = read_until('R');

    if !r.starts_with("\x1b[") {
        return Err(Error::from(TerminalError::InvalidVTSequence{sequence:r}));
    }

    let semi_index = r.find(';').ok_or(TerminalError::InvalidVTSequence{sequence:r.clone()})?;
    let height = usize::from_str_radix(&r[2..semi_index], 10)?;
    let width = usize::from_str_radix(&r[semi_index+1..r.len()-1], 10)?;
 
    Ok((width, height))
}
