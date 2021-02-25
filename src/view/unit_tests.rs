use super::*;

#[test]
fn new__clears_entire_terminal() {
    // Given
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;
    let expected_view_state = [[' '; WIDTH]; HEIGHT];
    // fake_terminal (mock, fake, spy, stub)
    let terminal_spy = TerminalSpy::new(WIDTH, HEIGHT);
    let sut = View::new;

    // When
    // Constructor dependency injection
    let res = sut(terminal_spy);

    // Then
    assert!(res.is_ok());
    // buffer() is not Position_Buffer, represents hardware"
    assert_eq!(res.unwrap().terminal().buffer(), expected_view_state);
}
