use std::vec;

use super::*;

#[test]
fn index__index_empty_buffer_yields_0() {
    // Given
    let pos = CursorPosition::default();
    let expected_res = 0;
    let sut = Utf8Buffer::default();

    // When
    let res = sut.index(pos);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
#[ignore]
fn index__index_populated_buffer_col_2_yields_2() {
    // Given
    let pos = CursorPosition::new(2, 0);
    let expected_res = 2;
    let sut = Utf8Buffer::from(vec![String::from("孫悟空")]);

    // When
    let res = sut.index(pos);

    // Then
    assert_eq!(res, expected_res);
}
