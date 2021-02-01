use super::*;

#[test]
fn append_a_row_to_newly_created_buffer_yields_a_buffer_with_2_rows() {
    // Given
    let expected_res = 2;
    let mut sut = Utf8Buffer::new();

    // When
    let res = sut.append_row();

    // Then
    assert_eq!(res.row_count(), expected_res);
}
