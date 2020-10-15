use super::*;

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
