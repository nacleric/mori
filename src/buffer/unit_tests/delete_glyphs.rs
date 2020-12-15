use super::*;

#[test]
fn delete_glyphs__remove_data_from_entire_buffer_yields_empty_buffer() {
    // Given
    let sentence = String::from("green sleeping mask");
    let expected_result = String::from("");
    let mut sut = Buffer::new();
    sut.insert_glyphs(sentence.chars());

    // When
    let res = sut.delete_glyphs();

    // Then
    assert_eq!(res.row_content(), expected_result.as_bytes());
}
