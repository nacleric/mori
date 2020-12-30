use super::*;

#[test]
fn check_col_state__checks_col_yields_beginning_of_line() {
    // Given
    let pos = Position::new(0, 1);
    let expected_col_state = ColumnState::BeginningOfLine;
    let sut = Buffer::from(vec![String::from(""),String::from("world")]);

    // When
    let res = sut.check_col_state(pos);

    // Then
    assert_eq!(res, expected_col_state);
}

#[test]
fn check_col_state__checks_col_yields_end_of_line() {
    // Given
    let pos = Position::new(4, 1);
    let expected_col_state = ColumnState::EndOfLine;
    let sut = Buffer::from(vec![String::from(""),String::from("world")]);

    // When
    let res = sut.check_col_state(pos);

    // Then
    assert_eq!(res, expected_col_state);
}

#[test]
fn check_col_state__checks_col_yields_middle_of_line() {
    // Given
    let pos = Position::new(2, 1);
    let expected_col_state = ColumnState::MiddleOfLine;
    let sut = Buffer::from(vec![String::from(""),String::from("world")]);

    // When
    let res = sut.check_col_state(pos);

    // Then
    assert_eq!(res, expected_col_state);
}

#[test]
fn check_col_state__checks_col_yields_invalid_position() {
    // Given
    let pos = Position::new(10, 1);
    let expected_col_state = ColumnState::InvalidPosition;
    let sut = Buffer::from(vec![String::from(""), String::from("world")]);

    // When
    let res = sut.check_col_state(pos);

    // Then
    assert_eq!(res, expected_col_state);
}

#[test]
fn check_row_state__checks_row_yields_lower_bound() {
    // Given
    let pos = Position::new(0, 2);
    let expected_row_state = RowState::LowerBound;
    let sut = Buffer::from(vec![String::from("hello"), String::from("world"), String::from("!")]);

    // When
    let res = sut.check_row_state(pos);

    // Then
    assert_eq!(res, expected_row_state);
}

#[test]
fn check_row_state__checks_row_yields_middle_bound() {
    // Given
    let pos = Position::new(0, 1);
    let expected_row_state = RowState::MiddleBound;
    let sut = Buffer::from(vec![String::from("hello"), String::from("world"), String::from("!")]);

    // When
    let res = sut.check_row_state(pos);

    // Then
    assert_eq!(res, expected_row_state);
}

#[test]
fn check_row_state__checks_row_yields_upper_bound() {
    // Given
    let pos = Position::new(0, 0);
    let expected_row_state = RowState::UpperBound;
    let sut = Buffer::from(vec![String::from("hello"), String::from("world"), String::from("!")]);

    // When
    let res = sut.check_row_state(pos);

    // Then
    assert_eq!(res, expected_row_state);
}

// ------------------Might not need---------------------------------
// TODO: account for rows out of boundary, ie: row[-1]
#[test]
fn check_lower_row__checks_if_lower_row_is_populated_yields_true() {
    // Given
    let pos = Position::new(0, 0);
    let expected_res = true;
    let sut = Buffer::from(vec![String::from("hello"), String::from("world")]);

    // When
    let res = sut.check_lower_row(pos);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn check_lower_row__checks_if_lower_row_is_populated_yields_false() {
    // Given
    let pos = Position::new(0, 0);
    let expected_res = false;
    let sut = Buffer::from(vec![String::from("hello"), String::from("")]);

    // When
    let res = sut.check_lower_row(pos);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn check__upper_row__checks_upper_row_is_populated_yields_true() {
    // Given
    let pos = Position::new(0, 1);
    let expected_res = true;
    let sut = Buffer::from(vec![String::from("hello"), String::from("world")]);

    // When
    let res = sut.check_upper_row(pos);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn check__upper_row__checks_upper_row_is_populated_yields_false() {
    // Given
    let pos = Position::new(0, 1);
    let expected_res = false;
    let sut = Buffer::from(vec![String::from(""), String::from("world")]);

    // When
    let res = sut.check_upper_row(pos);

    // Then
    assert_eq!(res, expected_res);
}
// -----------------------------------------------------------------