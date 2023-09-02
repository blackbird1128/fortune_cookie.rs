pub fn parse_fortune_string(fortune_string: &str) -> Vec<String> {
    let mut fortune_vec: Vec<String> = Vec::new();

    for fortune in fortune_string.split(['%']) {
        fortune_vec.push(fortune.trim().to_owned());
    }
    fortune_vec
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_fortune_string1() {
        let str_test = "hello world\n%\nfortune";
        assert_eq!(
            parse_fortune_string(str_test),
            vec!["hello world", "fortune"]
        );
    }
}
