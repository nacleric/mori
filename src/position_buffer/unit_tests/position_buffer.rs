use super::*;
use crate::cursor_position::CursorPosition;
use crate::utf8_buffer::Utf8Buffer;

#[test]
fn new__returns_position_buffer() {
    // Given
    let buf = Utf8Buffer::new();
    let pos = CursorPosition::default();
    let expected_res = PositionBuffer::new(Utf8Buffer::new(), CursorPosition::default());
    let sut = PositionBuffer::new;

    // When
    let res = sut(buf, pos);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn move_down_in_empty_buffer_yields_default_position() {
    // Given
    let expected_res: PositionBuffer<Utf8Buffer> = PositionBuffer::default();
    let sut: PositionBuffer<Utf8Buffer> = PositionBuffer::default();

    // When
    let mut res = sut;
    res.move_down();

    // Then
    assert_eq!(res.position, expected_res.position);
}

#[test]
fn move_down_in_col_0_row_0_yields_col_0_row_1() {
    // Given
    let expected_buffer = PositionBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(0, 1),
    );
    let sut = PositionBuffer::new(
        Utf8Buffer::from(vec![String::from("hello"), String::from("world")]),
        CursorPosition::new(0, 0),
    );

    // When
    let mut res = sut;
    res.move_down();

    // Then
    assert_eq!(res, expected_buffer);
}
