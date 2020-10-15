use super::*;

#[test]
fn update_position__update_position_within_empty_data_model() {
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
