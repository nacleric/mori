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
