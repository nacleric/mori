use super::*;

// Ask Brad about Constructor dependency injection
/*
#[test]
fn new__clears_entire_terminal() {
    // Given
    let expected_terminal_state = [[' '; WIDTH]; HEIGHT];
    // fake_terminal (mock, fake, spy, stub)
    let mock_terminal = MockTerminal::new();
    let sut = Terminal::new;

    // When
    // Note: Constructor dependency injection
    let res = sut(mock_terminal);

    // Then
    assert!(res.is_ok());
    // buffer() is not Position_Buffer, represents hardware"
    assert_eq!(res.unwrap().terminal().buffer(), expected_view_state);
}
*/
#[test]
fn new__creates_empty_terminal() {
    // Given
    let expected_terminal_state = [[None; WIDTH]; HEIGHT];
    // fake_terminal (mock, fake, spy, stub)
    let sut = MockTerminal::new;

    // When
    // Constructor dependency injection
    let res = sut();

    // Then
    // buffer() is not Position_Buffer, represents hardware"
    assert_eq!(res.content(), expected_terminal_state);
}

#[test]
fn clear__clears_populated_terminal() {
    // Given
    let expected_terminal_state = [[None; WIDTH]; HEIGHT];
    let sut = MockTerminal::new();

    // When
    let res = sut.clear();

    // Then
    assert_eq!(res, expected_terminal_state);
}