// Write a program that reads an input text file, looks for an occurence of a word, replace that word with another else, and write to that file.
// Inputs: file
// process: find the word utilize, and replace it with use
// outputs: write to the file.

fn read_file(filename: String) -> String {
    return "".to_string();
}
fn replace_occurence(text: String, find: String, replace: String) -> String {
    return "".to_string();
}
fn write_file(text: String, filename: String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let content: String = read_file("input.txt".to_string());
        assert_eq!(
            content,
            "One should never utilize the word \"utilize\" in writing. Use \"use\" instead."
        )
    }

    #[test]
    fn test_replace_occurence() {
        let content: String =
            "One should never utilize the word \"utilize\" in writing. Use \"use\" instead."
                .to_string();
        let expected: String =
            "One should never use the word \"use\" in writing. Use \"use\" instead.".to_string();
        assert_eq!(
            replace_occurence(content, "utilize".to_string(), "use".to_string()),
            expected
        );
    }
}

fn main() {
    println!("Hello World!")
}
