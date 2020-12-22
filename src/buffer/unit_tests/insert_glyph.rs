use super::*;

// TODO: this
// String not inserted into buffer
#[test]
fn insert_glyph__adding_a_glyph_to_an_empty_data_model_yields_a_model_containing_only_the_glyph(
) -> Result<()> {
    // Given
    let glyph = 'a';
    let expected_result = Buffer::from(vec![glyph.to_string()]);
    let pos = Position::default(); // Sets position to 0, 0
    let expected_pos = Position::new(1, 0);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyph(pos, glyph);

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_a_weird_glyph_to_an_empty_data_model_yields_a_model_containing_only_the_glyph(
) -> Result<()> {
    // Given
    let glyph = '你';
    let expected_result = Buffer::from(vec![glyph.to_string()]);
    let pos = Position::default();
    let expected_pos = Position::new(1, 0);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyph(pos, glyph);

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_an_escape_character_glyph_to_an_empty_data_model_yields_escape_glyph(
) -> Result<()> {
    // Given
    let glyph = '\n';
    let expected_result = Buffer::from(vec![glyph.to_string()]);
    let pos = Position::default();
    let expected_pos = Position::new(1, 0);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyph(pos, glyph); // maybe like this

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_a_tab_to_an_empty_data_model_yields_a_tab() -> Result<()> {
    // Given
    let glyph = '\t';
    let expected_result = Buffer::from(vec![glyph.to_string()]);
    let pos = Position::default();
    let expected_pos = Position::new(1, 0);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyph(pos, glyph);

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_result);
    Ok(())
}

#[test]
fn insert_glyph__adding_a_glyph_in_a_specific_position_yields_a_glyph_yields_inserted_glyph_in_correct_location() -> Result<()> {
    // Given
    let glyph = 'n';
    let expected_result= Buffer::from(vec![String::from("green sleeping mask")]);
    let pos = Position::new(4, 0);
    let expected_pos = Position::new(5, 0);
    let mut sut = Buffer::from(vec![String::from("gree sleeping mask")]);

    // When
    let res = sut.insert_glyph(pos, glyph);

    // Then
    assert_eq!(sut, expected_result);
    assert_eq!(res, expected_pos);
    Ok(())
}