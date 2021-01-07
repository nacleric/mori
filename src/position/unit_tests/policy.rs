use super::*;
use crate::Buffer;

#[test]
fn policy__empty_buffer_cursor_moves_left_yields_unchanged_position() {
    // Given
    let expected_res = Position::default();
    let sut = Position::default();
    
    // When
    let res = sut.move_left();

    // Then
    assert_eq!(res, expected_res);
}

// TODO: test for multi-byte character
#[test]
fn policy__populated_buffer_cursor_moves_left_from_col_0_row_1_yields_col_11_row_0() {
    // Given
    let buf = Buffer::from(vec![String::from("hello world"), String::from("")]);
    let expected_res = Position::new(11, 0);
    let sut = Position::new(0, 1);
    
    // When
    let res = sut.move_left();

    // Then
    assert_eq!(res, expected_res);
}