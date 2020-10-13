use crate::editor;

use std::fs;
use std::path::Path;
use std::io::Write;

// TODO: Handle unused variables
#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__read_file() {
        // Given
        let file= "test.txt";
        let contents = String::from("hello world");
        let mut testfile = fs::File::create(file);
        let testfile = match &mut testfile {
            Ok(testfile) => testfile.write_all(contents.as_bytes()),
            Err(error) => panic!("{:?}", error)
        };

        // When
        let res = editor::read_file(file);
        fs::remove_file(file).unwrap(); // Delete this later

        // Then
        assert_eq!(res, contents);
    }

    #[test]
    fn test__render_file() {
        // Given
        let file= "test.txt";
        let contents = String::from("hello world");
        let mut testfile = fs::File::create(file);
        let testfile = match &mut testfile {
            Ok(testfile) => testfile.write_all(contents.as_bytes()),
            Err(error) => panic!("{:?}", error)
        };

        // When
        let res = editor::read_file(file);
        fs::remove_file(file);

        editor::render_file(contents)
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod repl {
    use super::*;

    #[test]
    fn test__example() {
        editor::example();
    }

    #[test]
    fn test__create_file() -> std::io::Result<()> {
        // Given
        let filepath = "test.txt";

        // When
        fs::File::create(filepath)?;

        // Then
        Ok(())
    }

    #[test]
    fn test__remove_file() -> std::io::Result<()> {
        // Given
        let filepath = "test.txt";

        // When
        fs::remove_file(filepath)?;

        // Then
        Ok(())
    }

}