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
}

fn main() {
    println!("Hello World!")
}
