use super::*;

#[test]
fn set__new_buffer_has_position_of_0_0() {
    // Given
    let expected_res = Position::default();
    let sut = Buffer::new();

    // When
    let res = sut.pos();

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn set_pos__update_position_to_invalid_coords_fails() {
    // Given
    let invalid_col = 6;
    let invalid_pos = Position::new(invalid_col, 0);
    let expected_result = Err(Error::InvalidPosition(invalid_pos));
    let mut sut = Buffer::new();

    // When
    let res = sut.set_pos(invalid_pos);

    // Then
    assert_eq!(res, expected_result);
}
