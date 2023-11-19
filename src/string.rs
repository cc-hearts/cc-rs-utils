
/// Splits a string
///
/// This function takes a string and a delimiter as arguments, and then splits the string using the delimiter.
///
/// # Arguments
///
/// * `input_str` - The string to be split
/// * `delimiter` - The character to use as a delimiter for splitting the string
///
/// # Returns
///
/// Returns a vector of the split substrings
///
/// # Example
///
/// ```
/// use cc_rs_utils::string::split_string;
/// let v = split_string("Hello, World!", ',');
/// assert_eq!(v, vec!["Hello", " World!"]);
/// ```
pub fn split_string(input_str: &str, delimiter: char) -> Vec<&str> {
    input_str.split(delimiter).collect()
}

/// Capitalizes a string
///
/// This function takes a string as an argument, capitalizes the first character of the string, and returns the modified string.
///
/// # Arguments
///
/// * `input_str` - The string to be capitalized
///
/// # Returns
///
/// Returns the string with the first character capitalized
///
/// # Example
///
/// ```
/// use cc_rs_utils::string::capitalize;
/// let s = capitalize("hello");
/// assert_eq!(s, "Hello");
/// ```
pub fn capitalize(input_str: &str) -> String {
    format!("{}{}", &input_str[0..1].to_uppercase(), &input_str[1..])
}

/// Uncapitalizes a string
///
/// This function takes a string as an argument, converts the first character of the string to lowercase, and returns the modified string.
///
/// # Arguments
///
/// * `input_str` - The string to be uncapitalized
///
/// # Returns
///
/// Returns the string with the first character converted to lowercase
///
/// # Example
///
/// ```
/// use cc_rs_utils::string::un_capitalize;
/// let s = un_capitalize("Hello");
/// assert_eq!(s, "hello");
/// ```
pub fn un_capitalize(input_str: &str) -> String {
    format!("{}{}",&input_str[0..1].to_lowercase(), &input_str[1..])
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
}
