use super::*;

#[test]
fn return_selected_mut_row() {
    // Given
    let row_index = 0;
    let edit_row = &mut String::from("");
    let expected_res = Some(edit_row);
    let mut sut = Utf8Buffer::new();

    // When
    let res = sut.edit_row(row_index);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn return_selected_populated_mut_row() {
    // Given
    let row_index = 0;
    let edit_row = &mut String::from("hello world");
    let expected_res = Some(edit_row);
    let mut sut = Utf8Buffer::from(vec![String::from("hello world")]);

    // When
    let res = sut.edit_row(row_index);

    // Then
    assert_eq!(res, expected_res);
}

#[test]
fn return_selected_mut_grapheme_cluster() {
    // Given
    let row_index = 0;
    let edit_row = &mut String::from("孫悟空");
    let expected_res = Some(edit_row);
    let mut sut = Utf8Buffer::from(vec![String::from("孫悟空")]);

    // When
    let res = sut.edit_row(row_index);

    // Then
    assert_eq!(res, expected_res);
}
