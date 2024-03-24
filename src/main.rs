use std::fs::{read_to_string, File};
use std::io::prelude::*;

// Write a program that reads an input text file, looks for an occurence of a word, replace that word with another else, and write to that file.
// Inputs: file
// process: find the word utilize, and replace it with use
// outputs: write to the file.

fn read_file(file_path: String) -> String {
    let contents = read_to_string(file_path).expect("Should have been able to read the file.");
    return contents;
}
fn replace_occurence(text: &String, find: &String, replace: &String) -> String {
    return text.replace(find, replace);
}
fn write_file(text: &String, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{read_file, replace_occurence, write_file};

    #[test]
    fn test_read_file() {
        let content: String = read_file("content.txt".to_string());
        assert_eq!(
            content,
            "One should never utilize the word \"utilize\" in writing. Use \"use\" instead."
        )
    }

    #[test]
    fn test_replace_occurence() {
        let find = "utilize".to_string();
        let replace = "use".to_string();
        let content: String =
            "One should never utilize the word \"utilize\" in writing. Use \"use\" instead."
                .to_string();
        let expected: String =
            "One should never use the word \"use\" in writing. Use \"use\" instead.".to_string();
        assert_eq!(replace_occurence(&content, &find, &replace), expected);
    }

    #[test]
    fn test_write_file() {
        let content = "Hello World!".to_string();
        let output_path = "test.txt".to_string();
        write_file(&content, &output_path).expect("Should have been able to write to file");
        let read_content = read_file("test.txt".to_string());
        assert_eq!(content, read_content)
    }
}

fn main() {
    let input_path = "content.txt".to_string();
    let output_path = "output.txt".to_string();
    let find_word = "utilize".to_string();
    let replace_to = "use".to_string();
}
