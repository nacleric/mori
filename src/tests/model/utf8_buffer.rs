use crate::model::utf8_buffer::Utf8Buffer;
use crate::interfaces::RowBuffer;

#[test]
fn append_a_row_to_newly_created_buffer_yields_a_buffer_with_2_rows() {
    // Given
    let expected_res = 2;
    let mut sut = Utf8Buffer::new();

    // When
    let res = sut.append_row();

    // Then
    assert_eq!(res.row_count(), expected_res);
}

#[test]
fn deletes_empty_string_from_2nd_row_yields_deleted_string() {
    // Given
    let expected_row_count = 1;
    let expected_res = String::from("");
    let mut sut = Utf8Buffer::new();
    sut.append_row();

    // When
    let res = sut.delete_row(1).unwrap();

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut.row_count(), expected_row_count);
}

#[test]
fn deletes_populated_string_from_2nd_row_yields_deleted_string() {
    // Given
    let expected_row_count = 1;
    let expected_res = String::from("hello world");
    let mut sut = Utf8Buffer::from(vec![String::from(""), String::from("hello world")]);

    // When
    let res = sut.delete_row(1).unwrap();

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut.row_count(), expected_row_count);
}

#[test]
fn deletes_populated_grapheme_cluster_from_2nd_row_yields_deleted_string() {
    // Given
    let expected_row_count = 1;
    let expected_res = String::from("孫悟空");
    let mut sut = Utf8Buffer::from(vec![String::from(""), String::from("孫悟空")]);

    // When
    let res = sut.delete_row(1).unwrap();

    // Then
    assert_eq!(res, expected_res);
    assert_eq!(sut.row_count(), expected_row_count);
}

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

#[test]
fn newly_constructed_utf8_buffer_contains_1_row() {
    // Given
    let expected_res = 1;
    let sut = Utf8Buffer::new;

    // When
    let result = sut();

    // Then
    assert_eq!(result.row_count(), expected_res);
}