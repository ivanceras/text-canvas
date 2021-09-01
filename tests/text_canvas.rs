use text_canvas::TextCanvas;

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
