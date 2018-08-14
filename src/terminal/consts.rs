pub const ESC: &str = "\x1b";
pub const CSI: &str = "\x1b[";

pub const RESET_STYLE: &str = "0m";
pub const BRIGHT: &str = "1m";
pub const UNDERLINE: &str = "4m";
pub const NO_UNDERLINE: &str = "24m";
pub const NEGATIVE: &str = "7"; // swap foreground and background
pub const NO_NEGATIVE: &str = "27"; // removes negative

pub const BLACK: &str = "30m";
pub const RED: &str = "31m";
pub const GREEN: &str = "32m";
pub const YELLOW: &str = "33m";
pub const BLUE: &str = "34m";
pub const MAGENTA: &str = "35m";

pub const BRIGHT_RED: &str = "91m";


pub const CURSOR_UP: &str = "A";
pub const CURSOR_DOWN: &str = "B";
pub const CURSOR_LEFT: &str = "D";
pub const CURSOR_RIGHT: &str = "C";

pub const CURSOR_MOVE_TO: &str = "H";

pub const ERASE: &str = "X";

pub const ALTERNATE_SCREEN_BUFFER: &str = "?1049h";
pub const MAIN_SCREEN_BUFFER: &str = "?1049l";
