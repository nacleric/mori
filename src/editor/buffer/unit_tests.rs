#![allow(non_snake_case)]
use super::*;

#[test]
fn insert_glyph__adding_a_glyph_to_an_empty_data_model_yields_a_model_containing_only_the_glyph() -> Result<()>{
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
fn insert_escape_character__adding_an_escape_character_glyph_to_an_empty_data_model() -> Result<()> {
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
fn insert_tab___adding_a_tab_to_an_empty_data_model() -> Result<()> {
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
fn update_position__update_position_within_empty_data_model() {
    // Given
    let current_position = Buffer::new().pos;
    let expected_result = Position { col: 6, row: 0 };
    let mut sut = current_position;

    // When
    sut.update(6, 0);
    let res = sut;

    // Then
    assert_eq!(res, expected_result);
}

// Doesn't work yet
#[test]
fn delete_glyph__deleting_a_glyph_from_experimental_data_model() -> Result<()> {
    // Given
    let sentence = String::from("helloo world");
    let expected_result = String::from("hello world");
    let mut sut = Buffer::new();

    // When
    sut.pos.update(6, 0);
    let res = sut.delete_glyph();

    // Then
    assert_eq!(res, expected_result);
    Ok(())
}
