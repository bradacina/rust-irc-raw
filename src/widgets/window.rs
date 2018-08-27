use std::iter::Iterator;
use terminal::{consts, cursor, misc};

pub struct Window {
    pub Width: u16,
    pub Height: u16,
    pub PosRow: u16,
    pub PosCol: u16,
    buffer: String,
    pub wrap: bool,
    pub auto_scroll: bool
}

impl Window {
    pub fn new(width: u16, height: u16, posRow: u16, posCol: u16) -> Window {
        return Window {
            Width: width,
            Height: height,
            PosRow: posRow,
            PosCol: posCol,
            buffer: String::new(),
            wrap: false,
            auto_scroll: false
        };
    }

    pub fn add(&mut self, content: &str) {
        self.buffer.push_str(content);
    }

    pub fn draw(&mut self) {
        self.draw_margins();
        let lines = self.process();
        let height = self.Height as usize;

        let mut enough_lines = &lines[..];

        if self.auto_scroll && lines.len() > height {
            let split_at = lines.len() - (height - 1);
            let (_,enough_lines_split) = lines.split_at(split_at);
            enough_lines = enough_lines_split;
        }

        let mut line_count = 1;

        enough_lines.iter().for_each(|line| {
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

    fn draw_margins(&self) {
        misc::dec_line_drawing();

        let vert_line = str::repeat(
            &format!(
                "{}{}{}{}{}{}{}",
                consts::ESC,
                consts::CURSOR_SAVE_POS,
                consts::DEC_VERT_LINE,
                consts::ESC,
                consts::CURSOR_LOAD_POS,
                consts::ESC,
                consts::CURSOR_DOWN1
            ),
            (self.Height - 1) as usize,
        );

        cursor::move_to(self.PosRow, self.PosCol);
        print!(
            "{}{}{}",
            consts::DEC_TOP_LEFT_CORNER,
            str::repeat(consts::DEC_HORIZ_LINE, (self.Width - 2) as usize),
            consts::DEC_TOP_RIGHT_CORNER
        );

        cursor::move_to(self.PosRow + 1, self.PosCol);
        print!("{}", vert_line);

        cursor::move_to(self.PosRow + 1, self.PosCol + self.Width - 1);
        print!("{}", vert_line);

        cursor::move_to(self.PosRow + self.Height, self.PosCol);
        print!(
            "{}{}{}",
            consts::DEC_BOTTOM_LEFT_CORNER,
            str::repeat(consts::DEC_HORIZ_LINE, (self.Width - 2) as usize),
            consts::DEC_BOTTOM_RIGHT_CORNER
        );

        misc::ascii_chars_drawing();
    }

    fn process(&self) -> Vec<&str> {
        let mut start: i16 = -1;
        let mut count = 0; // number of characters seen for the current line
        let mut lines = vec![];
        let mut skip_until_newline = false;

        for (idx, ch) in self.buffer.char_indices() {
            if !skip_until_newline
            { 
                count += 1;
            }
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
                skip_until_newline = false;
                continue;
            }

            if count == self.Width - 2 || idx == self.buffer.len() - 1 {
                /*println!(
                    "pushing start {}, count {}, actual {}",
                    start,
                    count,
                    &self.buffer[(start + 1) as usize..idx + 1 as usize]
                );*/
                if !skip_until_newline {
                    lines.push(&self.buffer[(start + 1) as usize..idx + 1 as usize]);
                }

                start = idx as i16;
                count = 0;
                if !self.wrap {
                    skip_until_newline = true;
                }
                continue;
            }
        }

        lines
    }
}
