use super::*;

#[test]
fn policy__move_down_in_empty_buffer_yields_default_position() {
    // Given
    let pos = Position::default();
    let expected_res = Position::default();
    let expected_buffer = Buffer::default();
    let sut = Buffer::default();

    // When
    let res = sut.move_down(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_down_in_col_0_row_0_yields_col_0_row_1() {
    // Given
    let pos = Position::default();
    let expected_res = Position::new(0, 1);
    let expected_buffer = Buffer::from(vec![String::from("hi"), String::from("hi")]);
    let sut = Buffer::from(vec![String::from("hi"), String::from("hi")]);

    // When
    let res = sut.move_down(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_down_wierd_grapheme_in_col_0_row_0_yields_col_0_row_1() {
    // Given
    let pos = Position::default();
    let expected_res = Position::new(0, 1);
    let expected_buffer = Buffer::from(vec![String::from("孫悟空"), String::from("孫悟空")]);
    let sut = Buffer::from(vec![String::from("孫悟空"), String::from("孫悟空")]);

    // When
    let res = sut.move_down(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_down_checks_bottom_row_length_if_col_larger_yield_eol() {
    // Given
    let pos = Position::new(5, 0);
    let expected_res = Position::new(4, 1);
    let expected_buffer = Buffer::from(vec![String::from("hello"), String::from("mori")]);
    let sut = Buffer::from(vec![String::from("hello"), String::from("mori")]);

    // When
    let res = sut.move_down(pos);

    // Given
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_down_wierd_grapheme_checks_bottom_row_length_if_col_larger_yield_eol() {
    // Given
    let pos = Position::new(3, 0);
    let expected_res = Position::new(2, 1);
    let expected_buffer = Buffer::from(vec![String::from("孫悟空"), String::from("孫悟")]);
    let sut = Buffer::from(vec![String::from("孫悟空"), String::from("孫悟")]);

    // When
    let res = sut.move_down(pos);

    // Given
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_down_checks_bottom_row_length_if_col_shorter_yield_col_n_row_n_plus_1() {
    // Given
    let pos = Position::new(3, 0);
    let expected_res = Position::new(3, 1);
    let expected_buffer = Buffer::from(vec![String::from("hello"), String::from("mori")]);
    let sut = Buffer::from(vec![String::from("hello"), String::from("mori")]);

    // When
    let res = sut.move_down(pos);

    // Given
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_left_in_empty_buffer_yields_default_position() {
    // Given
    let pos = Position::default();
    let expected_res = Position::default();
    let expected_buffer = Buffer::default();
    let sut = Buffer::default();

    // When
    let res = sut.move_left(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer)
}

#[test]
fn policy__move_left_in_populated_buffer_yields_eol_row_0() {
    // Given
    let pos = Position::new(0, 1);
    let expected_res = Position::new(5, 0);
    let expected_buffer = Buffer::from(vec![String::from("hello"), String::from("")]);
    let sut = Buffer::from(vec![String::from("hello"), String::from("")]);

    // When
    let res = sut.move_left(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer)
}

#[test]
fn policy__move_right_in_empty_buffer_yields_default_position() {
    // Given
    let pos = Position::default();
    let expected_res = Position::default();
    let expected_buffer = Buffer::default();
    let sut = Buffer::default();

    // When
    let res = sut.move_right(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_right_in_populated_buffer_yields_bol_row_0() {
    // Given
    let pos = Position::default();
    let expected_res = Position::new(0, 1);
    let expected_buffer = Buffer::from(vec![String::from(""), String::from("hello")]);
    let sut = Buffer::from(vec![String::from(""), String::from("hello")]);

    // When
    let res = sut.move_right(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy_move_up_in_empty_buffer_yields_default_position() {
    // Given
    let pos = Position::default();
    let expected_res = Position::default();
    let expected_buffer = Buffer::default();
    let sut = Buffer::default();

    // When
    let res = sut.move_up(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy_move_up_in_col_0_row_1_yields_col_0_row_0() {    
    // Given
    let pos = Position::new(0, 1);
    let expected_res = Position::new(0, 0);
    let expected_buffer = Buffer::from(vec![String::from("hi"), String::from("hi")]);
    let sut = Buffer::from(vec![String::from("hi"), String::from("hi")]);

    // When
    let res = sut.move_up(pos);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_up_checks_upper_row_length_if_col_larger_yield_eol() {
    // Given
    let pos = Position::new(5, 1);
    let expected_res = Position::new(4, 0);
    let expected_buffer = Buffer::from(vec![String::from("mori"), String::from("hello")]);
    let sut = Buffer::from(vec![String::from("mori"), String::from("hello")]);

    // When
    let res = sut.move_up(pos);

    // Given
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn policy__move_up_checks_upper_row_length_if_col_shorter_yield_col_n_row_n_minus_1() {
    // Given
    let pos = Position::new(3, 1);
    let expected_res = Position::new(3, 0);
    let expected_buffer = Buffer::from(vec![String::from("mori"), String::from("hello")]);
    let sut = Buffer::from(vec![String::from("mori"), String::from("hello")]);

    // When
    let res = sut.move_up(pos);

    // Given
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_buffer);
}