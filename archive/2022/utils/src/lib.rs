use std::error::Error;
use std::fs;

pub fn read_input_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_file() {
        let file_path = "input.txt";
        let result = read_input_file(file_path).unwrap();
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_read_input_file_error() {
        let file_path = "input2.txt";
        let result = read_input_file(file_path);
        assert!(result.is_err());
    }
}

