use super::*;
use crate::view::mock_terminal::MockTerminalView;
use crate::view::unit_tests::dummy_adapter::TtyControlDummyAdapter;

// Ask Brad about Constructor dependency injection
#[test]
fn new__clears_entire_view() {
    // Given
    let expected_view_state = [[None; WIDTH]; HEIGHT];
    // fake_view (mock, fake, spy, stub)
    let mock_view = MockTerminalView::new();
    let mut sut = Terminal::new(mock_view, TtyControlDummyAdapter::new());

    // When
    sut.clear();

    // Then
    assert_eq!(sut.view_buffer().get_data(), expected_view_state);
}

// #[test]
// fn print__renders_graphemes_to_mockview() {
//     // Given
//     let expected_view_state = [[Some(' '); WIDTH]; HEIGHT];

//     // When
//     sut.print();
//     // Then
// }
