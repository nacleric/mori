use crate::model::cursor_position::CursorPosition;
use crate::model::editor_buffer::EditorBuffer;
use crate::model::utf8_buffer::{Direction, Utf8Buffer};

#[test]
fn backward_delete_grapheme_in_empty_buffer_returns_nothing() {
    // Given
    let expected_res = None;
    let mut sut = EditorBuffer::default();

    // When
    let res = sut.delete_grapheme(Direction::Backward);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn backward_delete_grapheme_in_sole_grapheme_returns_nothing() {
    // Given
    let grapheme = '4';
    let pos = CursorPosition::new(1, 0);
    let expected_res = Some('4');
    let mut sut = EditorBuffer::new(Utf8Buffer::from(vec![grapheme.to_string()]), pos);

    // When
    let res = sut.delete_grapheme(Direction::Backward);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn backward_delete_graphme_in_populated_buffer() {
    // Given
    let expected_res = Some('h');
    let expected_pos = CursorPosition::new(0, 0);
    let expected_buf = EditorBuffer::new(Utf8Buffer::from(vec![String::from("i")]), expected_pos);
    let mut sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hi")]),
        CursorPosition::new(1, 0),
    );

    // When
    let res = sut.delete_grapheme(Direction::Backward);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buf);
}

#[test]
fn forward_delete_grapheme_in_empty_buffer_returns_nothing() {
    // Given
    let expected_res = None;
    let mut sut = EditorBuffer::default();

    // When
    let res = sut.delete_grapheme(Direction::Forward);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn forward_delete_grapheme_in_sole_grapheme_returns_nothing() {
    // Given
    let grapheme = '4';
    let pos = CursorPosition::default();
    let expected_res = Some('4');
    let mut sut = EditorBuffer::new(Utf8Buffer::from(vec![grapheme.to_string()]), pos);

    // When
    let res = sut.delete_grapheme(Direction::Forward);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn forward_delete_graphme_in_populated_buffer() {
    // Given
    let expected_res = Some('i');
    let expected_pos = CursorPosition::new(1, 0);
    let expected_buf = EditorBuffer::new(Utf8Buffer::from(vec![String::from("h")]), expected_pos);
    let mut sut = EditorBuffer::new(Utf8Buffer::from(vec![String::from("hi")]), expected_pos);

    // When
    let res = sut.delete_grapheme(Direction::Forward);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buf);
}

#[test]
fn new__returns_editor_buffer() {
    // Given
    let buf = Utf8Buffer::new();
    let pos = CursorPosition::default();
    let expected_res = EditorBuffer::new(Utf8Buffer::new(), CursorPosition::default());
    let sut = EditorBuffer::new;

    // When
    let res = sut(buf, pos);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn move_down__in_empty_buffer_yields_default_position() {
    // Given
    let expected_res = EditorBuffer::default();
    let sut = EditorBuffer::default();

    // When
    let mut res = sut;
    res.move_down();

    // Then
    assert_eq!(res.position, expected_res.position);
}

#[test]
fn move_down__in_col_0_row_0_yields_col_0_row_1() {
    // Given
    let expected_res = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(0, 1),
    );
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(0, 0),
    );

    // When
    let mut res = sut;
    res.move_down();

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn move_left__in_empty_buffer_yields_default_position() {
    // Given
    let expected_res = EditorBuffer::default();
    let sut = EditorBuffer::default();

    // When
    let mut res = sut;
    res.move_left();

    // Then
    assert_eq!(res.position, expected_res.position);
}

#[test]
fn move_left__in_col_1_row_0_yields_col_0_row_0() {
    // Given
    let expected_res = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello")]),
        CursorPosition::default(),
    );
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello")]),
        CursorPosition::new(1, 0),
    );

    // When
    let mut res = sut;
    res.move_left();

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn move_left__in_col_0_row_1_yields_eol_row_0() {
    // Given
    let expected_res = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(5, 0),
    );
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(0, 1),
    );

    // When
    let mut res = sut;
    res.move_left();

    // Then
    assert_eq!(res, expected_res)
}

#[test]
fn move_right__in_empty_buffer_yields_default_position() {
    // Given
    let expected_res = EditorBuffer::default();
    let sut = EditorBuffer::default();

    // When
    let mut res = sut;
    res.move_right();

    // Then
    assert_eq!(res.position, expected_res.position);
}

#[test]
fn move_right__in_col_0_row_0_yields_col_1_row_0() {
    // Given
    let expected_res = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello")]),
        CursorPosition::new(1, 0),
    );
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello")]),
        CursorPosition::default(),
    );

    // When
    let mut res = sut;
    res.move_right();

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn move_right__in_eol_row_0_yields_bol_row_1() {
    // Given
    let expected_res = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(0, 1),
    );
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(5, 0),
    );

    // When
    let mut res = sut;
    res.move_right();

    // Then
    assert_eq!(res, expected_res)
}

#[test]
fn move_up__in_empty_buffer_yields_default_position() {
    // Given
    let expected_res = EditorBuffer::default();
    let sut = EditorBuffer::default();

    // When
    let mut res = sut;
    res.move_up();

    // Then
    assert_eq!(res.position, expected_res.position);
}

#[test]
fn move_up__in_col_0_row_1_yields_col_0_row_0() {
    // Given
    let expected_res = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::default(),
    );
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(0, 1),
    );

    // When
    let mut res = sut;
    res.move_up();

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn insert_grapheme_in_empty_buffer() {
    // Given
    let grapheme = 'h';
    let expected_pos = CursorPosition::new(1, 0);
    let expected_res =
        EditorBuffer::new(Utf8Buffer::from(vec![grapheme.to_string()]), expected_pos);
    let sut = EditorBuffer::default();

    // When
    let mut res = sut;
    res.insert_grapheme(grapheme);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn insert_grapheme_in_eol() {
    // Given
    let grapheme = 'i';
    let expected_pos = CursorPosition::new(2, 0);
    let expected_res = EditorBuffer::new(Utf8Buffer::from(vec![String::from("hi")]), expected_pos);
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("h")]),
        CursorPosition::new(1, 0),
    );

    // When
    let mut res = sut;
    res.insert_grapheme(grapheme);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn insert_grapheme_in_mol() {
    // Given
    let grapheme = 'i';
    let expected_pos = CursorPosition::new(2, 0);
    let expected_res = EditorBuffer::new(Utf8Buffer::from(vec![String::from("hi.")]), expected_pos);
    let sut = EditorBuffer::new(
        Utf8Buffer::from(vec![String::from("h.")]),
        CursorPosition::new(1, 0),
    );

    // When
    let mut res = sut;
    res.insert_grapheme(grapheme);

    // Then
    assert_eq!(res, expected_res);
}
