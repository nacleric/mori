use super::*;

#[test]
fn newly_constructed_utf8_buffer_contains_1_row() {
    // Given
    let expected_res = 1;
    let sut = Utf8Buffer::new;

    // When
    let result = sut();

    // Then
    assert_eq!(result.row_count(), expected_res);
}
