use super::*;

#[test]
fn clear__remove_data_from_entire_buffer_yields_empty_buffer() {
    // Given
    let sentence = String::from("green sleeping mask");
    let expected_result = String::from("");
    let mut sut = Buffer::new();
    sut.insert_graphemes(sentence.chars());

    // When
    let res = sut.delete_graphemes();

    // Then
    assert_eq!(res.row_content(), expected_result.as_bytes());
}
