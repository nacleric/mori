use super::*;

// Ask Brad about Constructor dependency injection
#[test]
fn new__clears_entire_view() {
    // Given
    let expected_view_state = [[None; WIDTH]; HEIGHT];
    // fake_view (mock, fake, spy, stub)
    let mock_view = MockTerminalView::new();
    let mut sut = Terminal::new(mock_view);

    // When
    sut.clear();

    // Then
    // assert_eq!(res.unwrap().view().view(), expected_view_state);
    assert_eq!(sut.view().contents(), expected_view_state);
}

// #[test]
// fn print__renders_graphemes_to_mockview() {
//     // Given
//     let expected_view_state = [[Some(' '); WIDTH]; HEIGHT];

//     // When
//     sut.print();
//     // Then
// }

