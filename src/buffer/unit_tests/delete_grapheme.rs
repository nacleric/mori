use super::*;

#[test]
fn delete_grapheme__backward_delete_an_empty_buffer_does_nothing() {
    // Given
    let pos = Position::default();
    let expected_pos = Position::default();
    let expected_opt_grapheme = None;
    let expected_buffer = Buffer::new();
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::new();

    // When
    let res = sut.delete_grapheme(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_grapheme__forward_delete_an_empty_buffer_does_nothing() {
    // Given
    let pos = Position::default();
    let expected_pos = Position::default();
    let expected_opt_grapheme = None;
    let expected_buffer = Buffer::new();
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::new();

    // When
    let res = sut.delete_grapheme(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

// Note: invalid position error because their is no space in front of the index to be able to delete. Maybe pad the buffer?
#[test]
fn delete_grapheme__backward_delete_sole_grapheme_returns_empty_buffer() {
    // Given
    let pos = Position::new(1, 0);
    let expected_pos = Position::default();
    let grapheme = 'a';
    let expected_opt_grapheme = Some(grapheme);
    let expected_buffer = Buffer::new();
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::from(vec![grapheme.to_string()]);

    // When
    let res = sut.delete_grapheme(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_grapheme__backward_delete_sole_weird_grapheme_returns_empty_buffer() {
    // Given
    let pos = Position::new(1, 0);
    let expected_pos = Position::default();
    let grapheme = '你';
    let expected_opt_grapheme = Some(grapheme);
    let expected_buffer = Buffer::new();
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::from(vec![grapheme.to_string()]);

    // When
    let res = sut.delete_grapheme(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_grapheme__forward_delete_sole_grapheme_returns_empty_buffer() {
    // Given
    let pos = Position::default();
    let expected_pos = Position::new(0, 0);
    let grapheme = 'a';
    let expected_opt_grapheme = Some(grapheme);
    let expected_buffer = Buffer::new();
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::from(vec![grapheme.to_string()]);

    // When
    let res = sut.delete_grapheme(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_grapheme__forward_delete_sole_weird_grapheme_returns_empty_buffer() {
    // Given
    let pos = Position::default();
    let expected_pos = Position::new(0, 0);
    let grapheme = '你';
    let expected_opt_grapheme = Some(grapheme);
    let expected_buffer = Buffer::new();
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::from(vec![grapheme.to_string()]);

    // When
    let res = sut.delete_grapheme(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

// Doesn't work yet
#[test]
fn delete_grapheme__backward_delete_from_data_model_removes_the_expected_grapheme() {
    // Given
    let pos = Position::new(6, 0);
    let expected_pos = Position::new(5, 0);
    let expected_buffer = Buffer::from(vec![String::from("green sleeping mask")]);
    let expected_opt_grapheme = Some('b');
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::from(vec![String::from("greenb sleeping mask")]);

    // When
    let res = sut.delete_grapheme(Direction::Backward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_grapheme__forward_delete_from_data_model_removes_the_expected_grapheme() {
    // Given
    let pos = Position::new(5, 0);
    let expected_pos = Position::new(5, 0);
    let expected_buffer = Buffer::from(vec![String::from("green sleeping mask")]);
    let expected_opt_grapheme = Some('b');
    let expected_res = (expected_pos, expected_opt_grapheme);
    let mut sut = Buffer::from(vec![String::from("greenb sleeping mask")]);

    // When
    let res = sut.delete_grapheme(Direction::Forward, pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

// #[test]
// fn delete_grapheme__forward_delete_end_of_line_n_pulls_in_line_n_plus_1() {
//     unimplemented!()
// }
