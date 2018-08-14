use super::consts::*;

pub fn cursor_up() {
    print!("{}{}", ESC, CURSOR_UP);
}

pub fn cursor_down() {
    print!("{}{}", ESC, CURSOR_DOWN);
}

pub fn cursor_left() {
    print!("{}{}", ESC, CURSOR_LEFT);
}

pub fn cursor_right() {
    print!("{}{}", ESC, CURSOR_RIGHT);
}

pub fn cursor_up_n(n: u16) {
    print!("{}{}{}", CSI, n, CURSOR_UP)
}

pub fn cursor_down_n(n: u16) {
    print!("{}{}{}", CSI, n, CURSOR_DOWN)
}

pub fn cursor_left_n(n: u16) {
    print!("{}{}{}", CSI, n, CURSOR_LEFT)
}

pub fn cursor_right_n(n: u16) {
    print!("{}{}{}", CSI, n, CURSOR_RIGHT)
}

pub fn move_to(row:u16, col:u16) {
    print!("{}{};{}{}", CSI, row, col, CURSOR_MOVE_TO)
}

pub fn erase(n: u16) {
    print!("{}{}{}", CSI, n, ERASE);
}