
pub fn split_string(input_str: &str, delimiter: char) -> Vec<&str> {
    input_str.split(delimiter).collect()
}

pub fn capitalize(input_str: &str) -> String {
    format!("{}{}", &input_str[0..1].to_uppercase(), &input_str[1..])
}

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
