use super::*;

// Will need policy to prevent off-by-1 errors eventually
#[test]
fn move_down__current_pos_col_4_row_2_yields_col_4_row_1() {
    // Given
    let expected_coord = (4, 1);
    let sut = Position::new(4, 2);

    // When
    let res = sut.move_up();

    // Then
    assert_eq!(res.as_tuple(), expected_coord);
}

#[test]
fn move_left__current_pos_col_4_row_2_yields_col_3_row_2() {
    // Given
    let expected_coord = (3, 2);
    let sut = Position::new(4, 2);

    // When
    let res = sut.move_left();

    // Then
    assert_eq!(res.as_tuple(), expected_coord);
}

#[test]
fn move_right__current_pos_col_4_row_2_yields_col_5_row_2() {
    // Given
    let expected_coord = (5, 2);
    let sut = Position::new(4, 2);

    // When
    let res = sut.move_right();

    // Then
    assert_eq!(res.as_tuple(), expected_coord);
}

#[test]
fn move_up__current_pos_col_4_row_2_yields_col_4_row_3() {
    // Given
    let expected_coord = (4, 3);
    let sut = Position::new(4, 2);

    // When
    let res = sut.move_down();

    // Then
    assert_eq!(res.as_tuple(), expected_coord);
}
