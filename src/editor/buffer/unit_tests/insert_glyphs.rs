use super::*;

#[test]
fn insert_glyphs__adding_multiple_glyphs_yields_model_with_a_sentence() {
    // Given
    let sentence = String::from("green sleeping mask");
    let expected_result = String::from("green sleeping mask");
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_glyphs(sentence.chars());

    // Then
    assert_eq!(res.contents(), expected_result.as_bytes());
}
