use std::iter::FromIterator;
use unicode_width::UnicodeWidthChar;

pub struct Cell {
    ch: char,
    /// width of this character
    width: usize,
}

pub struct Line {
    chars: Vec<Cell>,
    /// total width of this line
    width: usize,
}

impl Line {
    fn push_char(&mut self, ch: char) {
        self.chars.push(Cell {
            ch,
            width: ch.width().expect("must have a unicode width"),
        });
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            chars: vec![],
            width: 0,
        }
    }
}

pub struct StringBuffer {
    lines: Vec<Line>,
}

impl Default for StringBuffer {
    fn default() -> Self {
        Self { lines: vec![] }
    }
}

impl StringBuffer {
    /// the total number of lines in this string buffer
    fn total_lines(&self) -> usize {
        self.lines.len()
    }

    fn line_width(&self, n: usize) -> Option<usize> {
        self.lines.get(n).map(|l| l.width)
    }

    fn add_lines(&mut self, n: usize) {
        self.lines.push(Line::default());
    }

    fn add_col(&mut self, y: usize, n: usize) {
        for _i in 0..n {
            self.lines[y].push_char(' ');
        }
    }

    /// insert a character at this x and y and move chars after it to the right
    fn insert_char(&mut self, x: usize, y: usize, ch: char) {
        self.add_char(false, x, y, ch);
    }

    /// replace the character at this location
    fn replace_char(&mut self, x: usize, y: usize, ch: char) {
        self.add_char(true, x, y, ch);
    }
    fn add_char(&mut self, is_replace: bool, x: usize, y: usize, ch: char) {
        let line_gap = if y > self.total_lines() {
            y - self.total_lines()
        } else {
            0
        };
        if line_gap > 0 {
            self.add_lines(line_gap + 1);
        }
        let line = &self.lines[y];
        let col_diff = if x > line.width { x - line.width } else { 0 };
        if col_diff > 0 {
            self.add_col(y, col_diff + 1);
        }

        let ch_width = ch.width().expect("must have a unicode width");
        let cell = Cell {
            ch,
            width: ch_width,
        };

        if is_replace {
            self.lines[y].chars[x] = cell
        } else {
            self.lines[y].chars.insert(x, cell);
        }
    }
}

impl From<&str> for StringBuffer {
    fn from(s: &str) -> Self {
        let lines = s
            .lines()
            .map(|line| {
                let chars: Vec<Cell> = line
                    .chars()
                    .map(|ch| Cell {
                        width: ch.width().expect("must have a unicode width"),
                        ch,
                    })
                    .collect();

                Line {
                    width: chars.iter().map(|cell| cell.width).sum(),
                    chars,
                }
            })
            .collect();
        Self { lines }
    }
}

impl ToString for StringBuffer {
    fn to_string(&self) -> String {
        self.lines
            .iter()
            .map(|line| String::from_iter(line.chars.iter().map(|cell| cell.ch)))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_length() {
        let raw = "Hello world";
        let buffer = StringBuffer::from(raw);
        assert_eq!(buffer.total_lines(), 1);
        assert_eq!(buffer.line_width(0), Some(11));
    }

    #[test]
    fn lines_2() {
        let raw = "Hello\nworld";
        let buffer = StringBuffer::from(raw);
        assert_eq!(buffer.total_lines(), 2);
        assert_eq!(buffer.line_width(0), Some(5));
        assert_eq!(buffer.line_width(1), Some(5));
        assert_eq!(buffer.line_width(2), None);
    }

    #[test]
    fn cjk() {
        let raw = "Hello 文件系统";
        let buffer = StringBuffer::from(raw);
        assert_eq!(buffer.total_lines(), 1);
        assert_eq!(buffer.line_width(0), Some(14));
    }

    #[test]
    fn replace_start() {
        let raw = "Hello";
        let mut buffer = StringBuffer::from(raw);
        buffer.replace_char(0, 0, 'Y');
        assert_eq!(buffer.to_string(), "Yello");
    }

    #[test]
    fn replace_middle() {
        let raw = "Hello";
        let mut buffer = StringBuffer::from(raw);
        buffer.replace_char(2, 0, 'Y');
        assert_eq!(buffer.to_string(), "HeYlo");
    }

    #[test]
    fn replace_end() {
        let raw = "Hello";
        let mut buffer = StringBuffer::from(raw);
        buffer.replace_char(4, 0, 'Y');
        assert_eq!(buffer.to_string(), "HellY");
    }

    #[test]
    fn insert_start() {
        let raw = "Hello";
        let mut buffer = StringBuffer::from(raw);
        buffer.insert_char(0, 0, 'Y');
        assert_eq!(buffer.to_string(), "YHello");
    }

    #[test]
    fn insert_middle() {
        let raw = "Hello";
        let mut buffer = StringBuffer::from(raw);
        buffer.insert_char(2, 0, 'Y');
        assert_eq!(buffer.to_string(), "HeYllo");
    }

    #[test]
    fn insert_end() {
        let raw = "Hello";
        let mut buffer = StringBuffer::from(raw);
        buffer.insert_char(5, 0, 'Y');
        assert_eq!(buffer.to_string(), "HelloY");
    }
}
