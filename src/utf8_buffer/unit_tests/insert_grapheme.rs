use crate::utf8_buffer;

use super::*;

#[test]
fn adding_a_grapheme_to_an_empty_data_model_yields_a_model_containing_only_the_grapheme() {
    // Given
    let col_index = 0; // Sets position to 0, 0
    let row_index = 0;
    let expected_col = 0;
    let grapheme = 'a';
    let expected_row= Some(&mut grapheme.to_string());
    let expected_res = 0;
    let mut sut = Utf8Buffer::new().edit_row(row_index);

    // When
    let res = sut.insert_grapheme(col_index);

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut, expected_row);
}
