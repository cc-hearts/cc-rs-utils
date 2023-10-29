pub fn split_string(input_str: &str,delimiter: char) -> Vec<&str> {
    input_str.split(delimiter).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        let result = split_string("this is collect",' ');
        assert_eq!(result, vec!["this", "is", "collect"]);
    }
}
