use super::*;

#[test]
fn insert_graphemes__adding_multiple_graphemes_yields_model_with_a_sentence() {
    // Given
    let sentence = String::from("green sleeping mask");
    let expected_res = Buffer::from(vec![String::from("green sleeping mask")]);
    let pos = Position::default();
    let mut sut = Buffer::new();

    // When
    sut.insert_graphemes(pos, sentence.chars());

    // Then
    assert_eq!(sut, expected_res);
}

// #[test]
// fn insert_graphemes__adding_weird_graphemes_yields_model_with_a_sentence() {
//     // Given
//     let sentence = String::from("你好，世界");
//     let expected_result = String::from("你好，世界");
//     let mut sut = Buffer::new();

//     // When
//     let res = sut.insert_graphemes(sentence.chars());

//     // Then
//     assert_eq!(res.row_content(), expected_result.as_bytes());
// }
