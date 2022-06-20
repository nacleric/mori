use crate::model::cursor_position::CursorPosition;

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
    assert_eq!(res.col(), expected_col);
    assert_eq!(res.row(), expected_row);
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
    assert_eq!(res.col(), expected_col);
    assert_eq!(res.row(), expected_row);
    assert_eq!(res.as_tuple(), expected_tuple);
}
