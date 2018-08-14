use super::consts::*;

pub fn use_alternate_screen_buffer() {
    print!("{}{}", CSI, ALTERNATE_SCREEN_BUFFER);
}

pub fn use_main_screen_buffer() {
    print!("{}{}", CSI, MAIN_SCREEN_BUFFER);
}
