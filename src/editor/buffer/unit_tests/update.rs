use super::*;

#[test]
fn position__new_buffer_has_position_of_0_0() {
    // Given
    let expected_res = Position{ col: 0, row: 0 };
    let sut = Buffer::new();

    // When
    let res = sut.pos();

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn update_position__update_position_to_invalid_coords_fails() {
    // Given
    let current_position = Buffer::new().pos;
    let expected_result = Position { col: 6, row: 0 };
    let mut sut = current_position;

    // When
    sut.update(6, 0);
    let res = sut;

    // Then
    assert_eq!(res, expected_result);
}
