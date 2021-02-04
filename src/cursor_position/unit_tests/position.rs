use super::*;
use crate::{
    cursor_position::CursorPosition,
    position_buffer::PositionBuffer,
    utf8_buffer::Utf8Buffer,
};

#[test]
fn default__returns_col_0_row_0() {
    // Given
    let expected_col = 0;
    let expected_row = 0;
    let expected_tuple = (0, 0);
    let sut = CursorPosition::default;

    // When
    let res = sut();

    // Then
    assert_eq!(res.col, expected_col);
    assert_eq!(res.row, expected_row);
    assert_eq!(res.as_tuple(), expected_tuple);
}

#[test]
fn new__returns_expected_col_row() {
    // Given
    let expected_col = 4;
    let expected_row = 2;
    let expected_tuple = (4, 2);
    let sut = CursorPosition::new;

    // When
    let res = sut(expected_col, expected_row);

    // Then
    assert_eq!(res.col, expected_col);
    assert_eq!(res.row, expected_row);
    assert_eq!(res.as_tuple(), expected_tuple);
}

/*
// TODO: this
#[test]
fn move_down_in_given_position_yields_row_plus_1() {
    // Given
    let expected_res = CursorPosition::new(0, 1);
    let sut = CursorPosition::default();

    // When
    let res = sut.move_down(sut);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn move_left_in_given_position_yields_col_minus_1() {
    // Given
    let expected_res = CursorPosition::new(0, 0);
    let sut = CursorPosition::default();

    // When
    let res = sut.move_down(sut);

    // Then
    assert_eq!(res, expected_res);
}
*/