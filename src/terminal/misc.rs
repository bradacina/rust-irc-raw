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

pub fn query_cursor_pos() -> Result<(usize, usize), Error> {
    cursor::move_to(999, 999);
    print!("{}{}", CSI, QUERY_CURSOR_POS);
    stdout().flush();

    cursor::move_to(5,5);
    print!("sent query");
    stdout().flush();

    let mut r = String::new();
    let r_how_many = stdin().read_to_string(&mut r)?;
    
    cursor::move_to(6,5);
    print!("read query");
    stdout().flush();

    if !r.starts_with('[') {
        return Err(Error::from(TerminalError::InvalidVTSequence{sequence:r}));
    }

    let semi_index = r.find(';').ok_or(TerminalError::InvalidVTSequence{sequence:r.clone()})?;
    let width = usize::from_str_radix(&r[1..semi_index], 10)?;
    let height = usize::from_str_radix(&r[semi_index+1..r.len()-1], 10)?;
 
    Ok((width, height))
}
