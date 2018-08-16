use std::iter::Iterator;
use terminal::cursor;

pub struct Window {
    pub Width: u16,
    pub Height: u16,
    pub PosRow: u16,
    pub PosCol: u16,
    buffer: String,
}

impl Window {
    pub fn new(width: u16, height: u16, posRow: u16, posCol: u16) -> Window {
        return Window {
            Width: width,
            Height: height,
            PosRow: posRow,
            PosCol: posCol,
            buffer: String::new(),
        };
    }

    pub fn add(&mut self, content: &str) {
        self.buffer.push_str(content);
    }

    pub fn draw(&mut self) {
        let lines = self.proccess();

        let mut line_count = 0;

        lines.iter().for_each(|line| {
            if line_count >= self.Height {
                return;
            }

            cursor::move_to(self.PosRow + line_count, self.PosCol + 1);
            cursor::erase(self.Width - 2);
            cursor::move_to(self.PosRow + line_count, self.PosCol + 1);
            print!("{}", line);
            if line.len() < (self.Width - 2) as usize {
                print!(
                    "{:width$}",
                    "",
                    width = ((self.Width - 2) as usize) - line.len()
                );
            }
            line_count += 1;
        });
    }

    pub fn proccess(&self) -> Vec<&str> {
        let mut start: i16 = -1;
        let mut count = 0;
        let mut lines = vec![];

        for (idx, ch) in self.buffer.char_indices() {
            count += 1;
            //println!("looking at {}, start is {}, count is {}", ch, start, count);

            if ch == '\r' || ch == '\n' {
                if count > 1 {
                    /*println!(
                        "pushing start {}, count {}, actual {}",
                        start,
                        count,
                        &self.buffer[(start + 1) as usize..idx as usize]
                    );*/
                    lines.push(&self.buffer[(start + 1) as usize..idx as usize]);
                }
                start = idx as i16;
                count = 0;
                continue;
            }

            if count == self.Width - 2 || idx == self.buffer.len() -1 {
                /*println!(
                    "pushing start {}, count {}, actual {}",
                    start,
                    count,
                    &self.buffer[(start + 1) as usize..idx + 1 as usize]
                );*/
                lines.push(&self.buffer[(start + 1) as usize..idx + 1 as usize]);

                start = idx as i16;
                count = 0;
                continue;
            }
        }

        lines
    }
}
