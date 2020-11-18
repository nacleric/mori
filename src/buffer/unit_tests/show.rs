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
