use super::*;

#[test]
fn insert_glyph__adding_a_glyph_to_an_empty_data_model_yields_a_model_containing_only_the_glyph(
) -> Result<()> {
    // Given
    let glyph = 'a';
    let expected_result = String::from(glyph);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyph(glyph);

    // Then
    assert_eq!(res?.contents(), expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_an_escape_character_glyph_to_an_empty_data_model_yields_escape_glyph(
) -> Result<()> {
    // Given
    let escape_character_glyph = '\n';
    let expected_result = String::from(escape_character_glyph);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyph(escape_character_glyph);

    // Then
    assert_eq!(res?.contents(), expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_a_tab_to_an_empty_data_model_yields_a_tab() -> Result<()> {
    // Given
    let escape_character_glyph = '\t';
    let expected_result = String::from(escape_character_glyph);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyph(escape_character_glyph);

    // Then
    assert_eq!(res?.contents(), expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_multiple_glyphs_yields_model_with_a_sentence() -> Result<()> {
    // Given
    let sentence = String::from("green sleeping mask");
    let expected_result = String::from("green sleeping mask");
    let mut sut = Buffer::new();
    sut.pos.update(0, 0)?;

    // When
    let mut res = sut;
    for glyph in sentence.chars() {
        res.insert_glyph(glyph)?;
    }

    // Then
    assert_eq!(res.contents(), expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_a_glyph_in_a_specific_position_yields_a_glyph_yields_inserted_glyph_in_correct_location() -> Result<()> {
    // Given
    let glyph = 'n';
    let bad_sentence = String::from("gree sleeping mask");
    let expected_result = String::from("green sleeping mask");
    let mut sut = Buffer::new();
    for glyph in bad_sentence.chars() {
        sut.insert_glyph(glyph)?;
    }
    sut.pos.update(4, 0)?;

    // When
    let res = sut.insert_glyph(glyph);

    // Then
    assert_eq!(res?.contents(), expected_result);
    Ok(())
}