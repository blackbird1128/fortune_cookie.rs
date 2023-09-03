use crate::file_utils::FortuneResult;
use std::{thread, time::Duration};

pub fn parse_fortune_string(fortune_string: &str, file_path: &str) -> Vec<FortuneResult> {
    let mut fortune_vec: Vec<FortuneResult> = Vec::new();

    for fortune in fortune_string.split(['%']) {
        fortune_vec.push(FortuneResult {
            fortune: fortune.trim().to_owned(),
            file_path: file_path.to_owned(),
        });
    }
    fortune_vec
}

pub fn print_and_delay_size(message: &str, delay: bool) {
    println!("{message}");
    if delay {
        thread::sleep(Duration::from_millis((200 * message.len()) as u64))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_fortune_string1() {
        let str_test = "hello world\n%\nfortune";
        assert_eq!(
            parse_fortune_string(str_test, "test"),
            vec![
                FortuneResult {
                    fortune: "hello world".to_owned(),
                    file_path: "test".to_owned(),
                },
                FortuneResult {
                    fortune: "fortune".to_owned(),
                    file_path: "test".to_owned(),
                }
            ]
        );
    }
}
