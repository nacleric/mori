use super::*;

// Ask Brad about Constructor dependency injection
#[test]
fn new__clears_entire_view() {
    // Given
    let expected_view_state = [[Some(' '); WIDTH]; HEIGHT];
    // fake_view (mock, fake, spy, stub)
    let mock_view = MockView::new();
    let mut sut = View::new(mock_view);

    // When
    sut.clear();

    // Then
    // assert!(res.is_ok());
    // assert_eq!(res.unwrap().view().buffer(), expected_view_state);
    assert_eq!(sut.buffer().contents(), expected_view_state);
}
