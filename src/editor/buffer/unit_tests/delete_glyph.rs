use super::*;
#[test]
fn delete_glyph__deleting_an_empty_buffer_does_nothing() -> Result<()> {
    // Given
    let expected_result = String::new();
    let mut sut = Buffer::new();

    // When
    let res = sut.delete_glyph();

    // Then
    assert_eq!(res.contents(), expected_result);
    Ok(())
}

#[test]
fn delete_glyph__deleting_sole_glyph_at_valid_pos_returns_empty_buffer() -> Result<()> {
    // Given
    let glyph = 'a';
    let expected_result = String::new();
    let mut sut = Buffer::new();
    sut.insert_glyph(glyph)?;

    // When
    let res = sut.delete_glyph();

    // Then
    assert_eq!(res.contents(), expected_result);
    Ok(())
}

// Doesn't work yet
#[test]
fn delete_glyph__deleting_a_glyph_from_experimental_data_model_removes_the_expected_glyph(
) -> Result<()> {
    // Given
    let bad_sentence = String::from("greenb sleeping mask");
    let expected_result = String::from("green sleeping mask");
    let mut sut = Buffer::new();
    sut.set_data(bad_sentence);
    sut.pos.set(6, 0)?;

    // When
    let res = sut.delete_glyph();

    // Then
    assert_eq!(res.contents(), expected_result);
    Ok(())
}
