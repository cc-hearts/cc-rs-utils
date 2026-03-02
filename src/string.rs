
/// Splits `input_str` by `delimiter`, returning the resulting slices.
///
/// The returned slices borrow from `input_str`, so the caller must keep the
/// source string alive. Consecutive delimiters or leading/trailing delimiters
/// generate empty entries, matching `str::split` semantics.
pub fn split_string(input_str: &str, delimiter: char) -> Vec<&str> {
    input_str.split(delimiter).collect()
}

/// Capitalizes the first character of `input_str` and returns the owned result.
///
/// Empty strings return an empty `String`, and the remainder of the input is
/// appended unchanged. Multi-byte characters are handled through `char::to_uppercase`.
pub fn capitalize(input_str: &str) -> String {
    let mut chars = input_str.chars();
    let first = match chars.next() {
        Some(ch) => ch,
        None => return String::new(),
    };
    let mut result = first.to_uppercase().collect::<String>();
    result.push_str(chars.as_str());
    result
}

/// Lowercases the first character of `input_str` and returns the owned result.
///
/// Empty strings return an empty `String`, and the remainder is left untouched so
/// multi-byte characters are handled consistently with `capitalize`.
pub fn un_capitalize(input_str: &str) -> String {
    let mut chars = input_str.chars();
    let first = match chars.next() {
        Some(ch) => ch,
        None => return String::new(),
    };
    let mut result = first.to_lowercase().collect::<String>();
    result.push_str(chars.as_str());
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        let result = split_string("this is collect",' ');
        assert_eq!(result, vec!["this", "is", "collect"]);
    }

    #[test]
    fn test_capitalize() {
        let result = capitalize("valid_str");
        assert_eq!(result, "Valid_str")
    }

    #[test]
    fn test_un_capitalize() {
        let result = un_capitalize("This is apple.");
        assert_eq!(result, "this is apple.")
    }

    #[test]
    fn capitalize_empty_returns_empty() {
        assert!(capitalize("").is_empty());
    }

    #[test]
    fn un_capitalize_empty_returns_empty() {
        assert!(un_capitalize("").is_empty());
    }
}
