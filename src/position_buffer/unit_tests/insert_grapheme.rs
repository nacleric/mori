use super::*;
use crate::{cursor_position::CursorPosition, utf8_buffer::Utf8Buffer};

#[test]
fn insert_grapheme_in_empty_buffer() {
    // Given
    let grapheme = 'h';
    let expected_pos = CursorPosition::new(1, 0);
    let expected_res = PositionBuffer::new(
        Utf8Buffer::from(vec![grapheme.to_string()]),
        expected_pos,
    );
    let sut = PositionBuffer::default();

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
    let expected_res = PositionBuffer::new(
        Utf8Buffer::from(vec![String::from("hi")]),
        expected_pos,
    );
    let sut = PositionBuffer::new(
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
    let expected_res = PositionBuffer::new(
        Utf8Buffer::from(vec![String::from("hi.")]),
        expected_pos,
    );
    let sut = PositionBuffer::new(
        Utf8Buffer::from(vec![String::from("h.")]),
        CursorPosition::new(1, 0),
    );

    // When
    let mut res = sut;
    res.insert_grapheme(grapheme);

    // Then
    assert_eq!(res, expected_res);
}
