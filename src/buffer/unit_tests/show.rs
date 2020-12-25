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

// TODO: make compatible with new api
#[test]
#[ignore]
fn show__buffer_displays_a_single_grapheme() {
    // Given
    let expected_res = String::from("a").into_bytes();
    let mut buf = Buffer::new();
    let pos = Position::default();
    buf.insert_grapheme(pos, 'a');
    let mut stdout_mock = Vec::<u8>::new();

    // When
    let res = buf.show(&mut stdout_mock);

    //Then
    assert!(res.is_ok());
    assert_eq!(stdout_mock, expected_res);
}

#[test]
#[ignore]
fn show__buffer_displays_multiple_graphemes() {
    // Given
    let sentence = String::from("hello world");
    let mut buf = Buffer::new();
    let pos = Position::default();
    buf.insert_graphemes(pos, sentence.chars());
    let expected_res = sentence.into_bytes(); // self -> vec<u8> into_bytes() consumes sentence

    let mut stdout_mock = Vec::<u8>::new();

    // When
    let res = buf.show(&mut stdout_mock);

    //Then
    assert!(res.is_ok());
    assert_eq!(stdout_mock, expected_res);
}
