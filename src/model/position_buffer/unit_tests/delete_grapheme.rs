use crate::model::cursor_position::CursorPosition;
use crate::model::utf8_buffer::Utf8Buffer;

use super::*;

#[test]
fn backward_delete_grapheme_in_empty_buffer_returns_nothing() {
    // Given
    let expected_res = None;
    let mut sut = PositionBuffer::default();

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
    let mut sut = PositionBuffer::new(Utf8Buffer::from(vec![grapheme.to_string()]), pos);

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
    let expected_buf = PositionBuffer::new(Utf8Buffer::from(vec![String::from("i")]), expected_pos);
    let mut sut = PositionBuffer::new(
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
    let mut sut = PositionBuffer::default();

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
    let mut sut = PositionBuffer::new(Utf8Buffer::from(vec![grapheme.to_string()]), pos);

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
    let expected_buf = PositionBuffer::new(Utf8Buffer::from(vec![String::from("h")]), expected_pos);
    let mut sut = PositionBuffer::new(Utf8Buffer::from(vec![String::from("hi")]), expected_pos);

    // When
    let res = sut.delete_grapheme(Direction::Forward);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buf);
}
