use super::*;

#[test]
fn new__clears_entire_terminal() {
    // Given
    let expected_terminal_state = [[' '; WIDTH]; HEIGHT];
    // fake_terminal (mock, fake, spy, stub)
    let mock_terminal = MockTerminal::new();
    let sut = Terminal::new;

    // When
    // Constructor dependency injection
    let res = sut(mock_terminal);

    // Then
    assert!(res.is_ok());
    // buffer() is not Position_Buffer, represents hardware"
    assert_eq!(res.unwrap().terminal().buffer(), expected_view_state);
}
