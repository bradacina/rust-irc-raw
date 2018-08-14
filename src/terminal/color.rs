use super::consts::*;

pub fn red() {
    print!("{}{}", CSI, RED);
}

pub fn green() {
    print!("{}{}", CSI, GREEN);
}

pub fn underline() {
    print!("{}{}", CSI, UNDERLINE);
}

pub fn no_underline() {
    print!("{}{}", CSI, NO_UNDERLINE);    
}

pub fn negative() {
    print!("{}{}", CSI, NEGATIVE);    
}

pub fn no_negative() {
    print!("{}{}", CSI, NO_NEGATIVE);    
}

pub fn bright() {
    print!("{}{}", CSI, BRIGHT);
}

pub fn reset() {
    print!("{}{}", CSI, RESET_STYLE);
}

pub fn bright_red() {
    print!("{}{}", CSI, BRIGHT_RED);
    
}
