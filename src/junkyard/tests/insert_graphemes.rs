use super::*;

#[test]
fn insert_graphemes__adding_multiple_graphemes_yields_model_with_a_sentence() {
    // Given
    let pos = CursorPosition::default();
    let sentence = String::from("green sleeping mask");
    let expected_res = Utf8Buffer::from(vec![String::from("green sleeping mask")]);
    let mut sut = Utf8Buffer::new();

    // When
    sut.insert_graphemes(pos, sentence.chars());

    // Then
    assert_eq!(sut, expected_res);
}

// Fails because it needs to implemente grapheme_index
#[test]
#[ignore]
fn insert_graphemes__adding_weird_graphemes_yields_model_with_a_sentence() {
    // Given
    let pos = CursorPosition::default();
    let sentence = String::from("你好，世界");
    let expected_res = Utf8Buffer::from(vec![String::from("你好，世界")]);
    let mut sut = Utf8Buffer::new();

    // When
    sut.insert_graphemes(pos, sentence.chars());

    // Then
    assert_eq!(sut, expected_res);
}
