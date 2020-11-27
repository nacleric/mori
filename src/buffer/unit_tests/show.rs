use super::*;
use crate::interfaces::View;

#[test]
fn empty_buffer_displays_nothing() {
    // Given
    let expected_res = [];
    let buf = Buffer::new();
    let mut stdout_mock = Vec::<u8>::new();

    // When
    let res = buf.show(&mut stdout_mock);

    // Then
    assert!(res.is_ok());
    assert_eq!(stdout_mock, expected_res);
}

#[test]
fn show__buffer_displays_a_single_glyph() {
    // Given
    let expected_res = String::from("a").into_bytes();
    let mut buf = Buffer::new();
    buf.insert_glyph('a');
    let mut stdout_mock = Vec::<u8>::new();

    // When
    let res = buf.show(&mut stdout_mock);

    //Then
    assert!(res.is_ok());
    assert_eq!(stdout_mock, expected_res);
}

#[test]
fn show__buffer_displays_multiple_glyphs(){
    // Given
    let sentence = String::from("hello world");
    let mut buf = Buffer::new();
    buf.insert_glyphs(sentence.chars());
    let expected_res = sentence.into_bytes();

    let mut stdout_mock = Vec::<u8>::new();

    let buf = Buffer::new();
    // When
    let res = buf.show(&mut stdout_mock);

    //Then
    assert!(res.is_ok());
    assert_eq!(stdout_mock, expected_res);
}