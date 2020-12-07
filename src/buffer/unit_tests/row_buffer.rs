use super::*;

#[test]
fn new__instatiates_a_row_buffer_yields_a_buffer() {
    // Given
    let expected_result = vec![Buffer::new()];
    let sut = RowBuffer::new();

    // When
    let res = sut;

    // Then
    assert_eq!(res, expected_result);
}

#[test]
fn add_row__pushes_a_new_empty_buffer_into_rowbuffer_yields_two_empty_buffers() {
    // Given
    // When
    // Then
    unimplemented!();
}

#[test]
fn delete_row() {
    // Given
    // When
    // Then
    unimplemented!();
}
