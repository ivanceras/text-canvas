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

pub struct TextCanvas {
    lines: Vec<Line>,
}

impl Default for TextCanvas {
    fn default() -> Self {
        Self { lines: vec![] }
    }
}

impl TextCanvas {
    /// the total number of lines in this string buffer
    fn total_lines(&self) -> usize {
        self.lines.len()
    }

    fn line_width(&self, n: usize) -> Option<usize> {
        self.lines.get(n).map(|l| l.width)
    }

    fn add_lines(&mut self, n: usize) {
        for _i in 0..n {
            self.lines.push(Line::default());
        }
    }

    fn add_col(&mut self, y: usize, n: usize) {
        let ch = ' ';
        for _i in 0..n {
            println!("adding to line {}: {:?}", y, ch);
            self.lines[y].push_char(ch);
            self.lines[y].width += ch.width().expect("must have a unicode size");
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

    /// Add a character at x and y location, character widths are taken into account
    /// So if a 2 wide character `文` is in line 0, the coordinate (0,0) and (0,1)
    /// access the same character. If you need to insert a character next to this character
    /// you need to insert at (2,0).
    /// # Example`
    /// ```rust
    /// use text_canvas::TextCanvas;
    ///
    /// let mut buffer = TextCanvas::from("c文");
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
        dbg!(&line_gap);
        if self.total_lines() == 0 {
            self.add_lines(1);
        }
        if line_gap > 0 {
            self.add_lines(line_gap);
        }
        dbg!(&self.lines);
        let line = &self.lines[y];
        let col_diff = if x > line.width { x - line.width } else { 0 };
        dbg!(&col_diff);
        if col_diff > 0 {
            self.add_col(y, col_diff);
        }

        let ch_width = ch.width().expect("must have a unicode width");
        let cell = Cell {
            ch,
            width: ch_width,
        };
        dbg!(&x);
        dbg!(&self.lines);
        dbg!(&self.lines[y].width);
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

impl From<&str> for TextCanvas {
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

impl ToString for TextCanvas {
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
        let buffer = TextCanvas::from(raw);
        assert_eq!(buffer.total_lines(), 1);
        assert_eq!(buffer.line_width(0), Some(11));
    }

    #[test]
    fn insert_anywhere_col() {
        let mut buffer = TextCanvas::default();
        buffer.insert_char(5, 0, 'Y');
        assert_eq!(buffer.to_string(), "     Y");
    }

    #[test]
    fn insert_anywhere_line() {
        let mut buffer = TextCanvas::default();
        buffer.insert_char(0, 5, 'Y');
        assert_eq!(buffer.to_string(), "\n\n\n\n\nY");
    }

    #[test]
    fn lines_2() {
        let raw = "Hello\nworld";
        let buffer = TextCanvas::from(raw);
        assert_eq!(buffer.total_lines(), 2);
        assert_eq!(buffer.line_width(0), Some(5));
        assert_eq!(buffer.line_width(1), Some(5));
        assert_eq!(buffer.line_width(2), None);
    }

    #[test]
    fn cjk() {
        let raw = "Hello 文件系统";
        let buffer = TextCanvas::from(raw);
        assert_eq!(buffer.total_lines(), 1);
        assert_eq!(buffer.line_width(0), Some(14));
    }

    #[test]
    fn insert_end_cjk() {
        let raw = "Hello 文件系统";
        let mut buffer = TextCanvas::from(raw);
        buffer.insert_char(13, 0, 'Y');
        assert_eq!(buffer.to_string(), "Hello 文件系统Y");
    }

    #[test]
    fn insert_end_cjk_same_insert_on_13th_or_14th() {
        let raw = "Hello 文件系统";
        let mut buffer = TextCanvas::from(raw);
        buffer.insert_char(14, 0, 'Y');
        assert_eq!(buffer.to_string(), "Hello 文件系统Y");
    }

    #[test]
    fn insert_end_cjk_but_not_15th() {
        let raw = "Hello 文件系统";
        let mut buffer = TextCanvas::from(raw);
        buffer.insert_char(15, 0, 'Y');
        assert_eq!(buffer.to_string(), "Hello 文件系统 Y");
    }

    #[test]
    fn replace_start() {
        let raw = "Hello";
        let mut buffer = TextCanvas::from(raw);
        buffer.replace_char(0, 0, 'Y');
        assert_eq!(buffer.to_string(), "Yello");
    }

    #[test]
    fn replace_middle() {
        let raw = "Hello";
        let mut buffer = TextCanvas::from(raw);
        buffer.replace_char(2, 0, 'Y');
        assert_eq!(buffer.to_string(), "HeYlo");
    }

    #[test]
    fn replace_end() {
        let raw = "Hello";
        let mut buffer = TextCanvas::from(raw);
        buffer.replace_char(4, 0, 'Y');
        assert_eq!(buffer.to_string(), "HellY");
    }

    #[test]
    fn insert_start() {
        let raw = "Hello";
        let mut buffer = TextCanvas::from(raw);
        buffer.insert_char(0, 0, 'Y');
        assert_eq!(buffer.to_string(), "YHello");
    }

    #[test]
    fn insert_middle() {
        let raw = "Hello";
        let mut buffer = TextCanvas::from(raw);
        buffer.insert_char(2, 0, 'Y');
        assert_eq!(buffer.to_string(), "HeYllo");
    }

    #[test]
    fn insert_end() {
        let raw = "Hello";
        let mut buffer = TextCanvas::from(raw);
        buffer.insert_char(5, 0, 'Y');
        assert_eq!(buffer.to_string(), "HelloY");
    }
}
