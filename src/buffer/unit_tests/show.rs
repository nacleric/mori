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

// Ask Brad: public vs private api. Would opening a file and loading the contents still apply to testing public api, technically not mimicing what a user would actually do.
#[test]
fn show__buffer_displays_a_single_glyph() {
    // Given
    let expected_res = [String::from("a").into_bytes()];
    let buf = Buffer::new();
    buf.insert_glyph('a');
    let mut stdout_mock = Vec::<Vec<u8>>::new();

    // When
    let res = buf.show(&mut stdout_mock);

    //Then
    assert!(res.is_ok());
    assert_eq!(stdout_mock, expected_res);
}

fn show__buffer_inserts_a_glyph_and_displays_buffer(){
    unimplemented!()
}