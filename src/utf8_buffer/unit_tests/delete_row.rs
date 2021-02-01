use super::*;

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