use super::*;

#[test]
fn delete_glyphs__remove_data_from_entire_buffer_yields_empty_buffer() -> Result<()> {
    // Given
    let sentence = String::from("green sleeping mask");
    let expected_result = String::from("");
    let mut sut = Buffer::new();
    sut.set_data(sentence);

    // When
    let res = sut.delete_glyphs();

    // Then
    assert_eq!(res.contents(), expected_result);
    Ok(())
}