use std::iter::Iterator;
use terminal::cursor;

pub struct Window<'a> {
    pub Width: u16,
    pub Height: u16,
    pub PosRow: u16,
    pub PosCol: u16,
    buffer: String,
    lines: Vec<&'a str>,
    needs_processing: bool,
}

impl<'a> Window<'a> {
    pub fn new(width: u16, height: u16, posRow: u16, posCol: u16) -> Window<'a> {
        return Window {
            Width: width,
            Height: height,
            PosRow: posRow,
            PosCol: posCol,
            buffer: String::new(),
            lines: Window::init_vec(),//Vec::new(),
            needs_processing: false,
        };
    }

    fn init_vec() -> Vec<&'a str> {
        vec!["a1234567890qwdqwdwqd","b", "c", "d"]
    }

    pub fn add(&mut self, content: &str) {
        self.buffer.push_str(content);
    }

    pub fn draw(&mut self) {
        if self.needs_processing {
            self.proccess();
        }

        let mut line_count = 0;
        self.lines.iter().for_each(|line| {
            if line_count >= self.Height {
                return;
            }

            cursor::erase(self.Width);
            cursor::move_to(self.PosRow + line_count, self.PosCol);
            print!("{}", line);
            line_count += 1;
        });
    }

    pub fn proccess(&mut self) {}
}
