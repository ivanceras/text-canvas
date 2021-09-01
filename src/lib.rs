use std::iter::FromIterator;
use unicode_width::UnicodeWidthChar;

#[derive(Debug)]
pub struct Cell {
    ch: char,
    /// width of this character
    width: usize,
}

#[derive(Debug)]
pub struct Line {
    cells: Vec<Cell>,
    /// total width of this line
    width: usize,
}

impl Line {
    fn push_char(&mut self, ch: char) {
        self.cells.push(Cell {
            ch,
            width: ch.width().expect("must have a unicode width"),
        });
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            cells: vec![],
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

    /// break at line y and put the characters after x on the next line
    fn insert_line(&mut self, x: usize, y: usize) {}

    /// insert a character at this x and y and move cells after it to the right
    pub fn insert_char(&mut self, x: usize, y: usize, ch: char) {
        self.add_char(false, x, y, ch);
    }

    /// replace the character at this location
    pub fn replace_char(&mut self, x: usize, y: usize, ch: char) {
        self.add_char(true, x, y, ch);
    }

    /// TODO: take into account the widths of each cell
    /// Add a character at x and y location, character widths are taken into account
    /// So if a 2 wide character `文` is in line 0, the coordinate (0,0) and (0,1)
    /// access the same character. If you need to insert a character next to this character
    /// you need to insert at (2,0).
    /// # Example`
    /// ```rust
    /// use string_buffer::StringBuffer;
    ///
    /// let mut buffer = StringBuffer::from("c文");
    /// buffer.insert_char(2, 0, 'Y');
    /// assert_eq!(buffer.to_string(), "c文Y");
    /// ```
    fn add_char(&mut self, is_replace: bool, x: usize, y: usize, ch: char) {
        assert!(
            ch != '\n',
            "line breaks should have been pre-processed before this point"
        );
        assert!(
            ch != '\t',
            "tabs should have been pre-processed before this point"
        );
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
        dbg!(&col_diff);
        if col_diff > 0 {
            self.add_col(y, col_diff + 1);
        }

        let ch_width = ch.width().expect("must have a unicode width");
        let cell = Cell {
            ch,
            width: ch_width,
        };
        assert!(x <= self.lines[y].width);

        let char_index = Self::calc_col_insert_position(&self.lines[y], x);

        if is_replace {
            self.lines[y].cells[char_index] = cell
        } else {
            self.lines[y].cells.insert(char_index, cell);
        }
    }

    fn calc_col_insert_position(line: &Line, x: usize) -> usize {
        let mut col_width = 0;
        for (i, cell) in line.cells.iter().enumerate() {
            if col_width >= x {
                return i;
            }
            col_width += cell.width;
        }
        line.cells.len()
    }
}

impl From<&str> for StringBuffer {
    fn from(s: &str) -> Self {
        let lines = s
            .lines()
            .map(|line| {
                let cells: Vec<Cell> = line
                    .chars()
                    .map(|ch| Cell {
                        width: ch.width().expect("must have a unicode width"),
                        ch,
                    })
                    .collect();

                Line {
                    width: cells.iter().map(|cell| cell.width).sum(),
                    cells,
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
            .map(|line| String::from_iter(line.cells.iter().map(|cell| cell.ch)))
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
