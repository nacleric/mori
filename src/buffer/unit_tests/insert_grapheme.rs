use super::*;

// TODO: this
// String not inserted into buffer
#[test]
fn insert_grapheme__adding_a_grapheme_to_an_empty_data_model_yields_a_model_containing_only_the_grapheme(
) -> Result<()> {
    // Given
    let pos = Position::default(); // Sets position to 0, 0
    let expected_pos = Position::new(1, 0);
    let grapheme = 'a';
    let expected_res = Buffer::from(vec![grapheme.to_string()]);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_grapheme(pos, grapheme);

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_res);
    Ok(())
}

#[test]
fn insert_grapheme__adding_a_weird_grapheme_to_an_empty_data_model_yields_a_model_containing_only_the_grapheme(
) -> Result<()> {
    // Given
    let pos = Position::default();
    let expected_pos = Position::new(1, 0);
    let grapheme = '你';
    let expected_res = Buffer::from(vec![grapheme.to_string()]);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_grapheme(pos, grapheme);

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_res);
    Ok(())
}

#[test]
fn insert_grapheme__adding_an_escape_character_grapheme_to_an_empty_data_model_yields_escape_grapheme(
) -> Result<()> {
    // Given
    let grapheme = '\n';
    let expected_res = Buffer::from(vec![grapheme.to_string()]);
    let pos = Position::default();
    let expected_pos = Position::new(1, 0);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_grapheme(pos, grapheme); // maybe like this

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_res);
    Ok(())
}

#[test]
fn insert_grapheme__adding_a_tab_to_an_empty_data_model_yields_a_tab() -> Result<()> {
    // Given
    let grapheme = '\t';
    let expected_res = Buffer::from(vec![grapheme.to_string()]);
    let pos = Position::default();
    let expected_pos = Position::new(1, 0);
    let mut sut = Buffer::new();

    // When
    let res = sut.insert_grapheme(pos, grapheme);

    // Then
    assert_eq!(res, expected_pos);
    assert_eq!(sut, expected_res);
    Ok(())
}

#[test]
fn insert_grapheme__adding_a_grapheme_in_a_specific_position_yields_a_grapheme_yields_inserted_grapheme_in_correct_location(
) -> Result<()> {
    // Given
    let pos = Position::new(4, 0);
    let expected_pos = Position::new(5, 0);
    let grapheme = 'n';
    let expected_res = Buffer::from(vec![String::from("green sleeping mask")]);
    let mut sut = Buffer::from(vec![String::from("gree sleeping mask")]);

    // When
    let res = sut.insert_grapheme(pos, grapheme);

    // Then
    assert_eq!(sut, expected_res);
    assert_eq!(res, expected_pos);
    Ok(())
}

#[test]
fn insert_grapheme__adding_a_weird_grapheme_in_a_specific_position_yields_a_grapheme_yields_inserted_grapheme_in_correct_location(
) -> Result<()> {
    // Given
    let pos = Position::new(1, 0);
    let expected_pos = Position::new(2, 0);
    let grapheme = '悟';
    let expected_res = Buffer::from(vec![String::from("孫悟空")]);
    let mut sut = Buffer::from(vec![String::from("孫空")]);

    // When
    let res = sut.insert_grapheme(pos, grapheme);

    // Then
    assert_eq!(sut, expected_res);
    assert_eq!(res, expected_pos);
    Ok(())
}