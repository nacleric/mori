use crate::model::cursor_position::CursorPosition;
use crate::model::utf8_buffer::Utf8Buffer;

use super::*;

#[test]
fn new__returns_position_buffer() {
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
