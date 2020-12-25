use super::*;

#[test]
fn insert_row__adding_a_row_to_buffer_yields_a_buffer_with_2_rows() {
    // Given
    let expected_result = Buffer::from(vec![String::new(), String::new()]);
    let pos = Position::default();
    let expected_pos = Position::new(0, 1);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_row(pos);

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_result);
}

/*
#[test]
fn insert_row__insert_row_in_the_middle_of_a_string_yields_new_row_with_populated_string() {
    // Ex: "hello world"
    //      hel
    //      lo world
    unimplemented!()
}
*/
