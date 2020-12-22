use super::*;
#[test]
fn delete_glyph__backward_delete_an_empty_buffer_does_nothing() {
    // Given
    let expected_opt_glyph = None;
    let expected_buffer = Buffer::new();
    let pos = Position::default();
    let expected_pos = Position::default();
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::new();

    // When
    let res = sut.delete_glyph(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_glyph__forward_delete_an_empty_buffer_does_nothing() {
    // Given
    let expected_opt_glyph = None;
    let expected_buffer = Buffer::new();
    let pos = Position::default();
    let expected_pos = Position::default();
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::new();

    // When
    let res = sut.delete_glyph(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

// Note: invalid position error because their is no space in front of the index to be able to delete. Maybe pad the buffer?
#[test]
fn delete_glyph__backward_delete_sole_glyph_returns_empty_buffer() {
    // Given
    let glyph = 'a';
    let expected_opt_glyph = Some(glyph);
    let expected_buffer = Buffer::new();
    let pos = Position::new(1, 0);
    let expected_pos = Position::default();
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::from(vec![glyph.to_string()]);

    // When
    let res = sut.delete_glyph(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_glyph__backward_delete_sole_weird_glyph_returns_empty_buffer() {
    // Given
    let glyph = '你';
    let expected_opt_glyph = Some(glyph);
    let expected_buffer = Buffer::new();
    let pos = Position::new(1, 0);
    let expected_pos = Position::default();
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::from(vec![glyph.to_string()]);

    // When
    let res = sut.delete_glyph(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_glyph__forward_delete_sole_glyph_returns_empty_buffer() {
    // Given
    let glyph = 'a';
    let expected_opt_glyph = Some(glyph);
    let expected_buffer = Buffer::new();
    let pos = Position::default();
    let expected_pos = Position::new(0, 0);
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::from(vec![glyph.to_string()]);

    // When
    let res = sut.delete_glyph(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_glyph__forward_delete_sole_weird_glyph_returns_empty_buffer() {
    // Given
    let glyph = '你';
    let expected_opt_glyph = Some(glyph);
    let expected_buffer = Buffer::new();
    let pos = Position::default();
    let expected_pos = Position::new(0, 0);
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::from(vec![glyph.to_string()]);

    // When
    let res = sut.delete_glyph(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

// Doesn't work yet
#[test]
fn delete_glyph__backward_delete_from_data_model_removes_the_expected_glyph() {
    // Given
    let expected_buffer = Buffer::from(vec![String::from("green sleeping mask")]);
    let expected_opt_glyph = Some('b');
    let pos = Position::new(6, 0);
    let expected_pos = Position::new(5, 0);
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::from(vec![String::from("greenb sleeping mask")]);

    // When
    let res = sut.delete_glyph(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_glyph__forward_delete_from_data_model_removes_the_expected_glyph() {
    // Given
    let expected_buffer = Buffer::from(vec![String::from("green sleeping mask")]);
    let expected_opt_glyph = Some('b');
    let pos = Position::new(5, 0);
    let expected_pos = Position::new(5, 0);
    let expected_result = (expected_pos, expected_opt_glyph);
    let mut sut = Buffer::from(vec![String::from("greenb sleeping mask")]);

    // When
    let res = sut.delete_glyph(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_result);
    assert_eq!(sut, expected_buffer);
}

// #[test]
// fn delete_glyph__forward_delete_end_of_line_n_pulls_in_line_n_plus_1() {
//     unimplemented!()
// }