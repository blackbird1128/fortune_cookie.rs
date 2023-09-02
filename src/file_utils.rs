use std::fs;

use regex::Regex;
use walkdir::WalkDir;

use crate::{fortune, percentage};

#[derive(Debug, PartialEq)]
pub struct FileContribution {
    pub file_path: String,
    pub percentage: u8,
}

pub fn get_fortune_files(vec_folders: &[&str]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for folder in vec_folders {
        for file in WalkDir::new(folder)
            .into_iter()
            .filter_map(|file| file.ok())
        {
            if file.metadata().unwrap().is_file() {
                files.push(file.path().to_str().unwrap().to_owned());
            }
        }
    }
    files
}
pub fn get_fortunes_from_file(file_path: &str) -> Result<Vec<String>, String> {
    let file_contents = fs::read_to_string(file_path);
    match file_contents {
        Ok(x) => Ok(fortune::parse_fortune_string(&x)),
        Err(_) => Err(format!("File not found: {}", file_path)),
    }
}

pub fn file_args_to_file_contribution(args: &str) -> Result<Vec<FileContribution>, String> {
    let re = Regex::new(r"(\d\d?%?\s)?(\S+)+").unwrap();
    let mut file_structs: Vec<FileContribution> = Vec::new();
    re.captures_iter(&args).for_each(|cap| {
        let mut cur_struct = FileContribution {
            percentage: 0,
            file_path: cap.get(2).unwrap().as_str().to_owned(),
        };
        cur_struct.percentage = match cap.get(1) {
            Some(x) => {
                let x = x.as_str().trim().replace("%", "");
                x.as_str().trim().parse().unwrap_or(0)
            }
            None => 0,
        };
        file_structs.push(cur_struct);
    });
    match percentage::fill_contributions(&mut file_structs) {
        Ok(_) => {}
        Err(x) => return Err(x),
    }
    Ok(file_structs)
}

#[cfg(test)]
mod tests {
    use crate::file_utils::{file_args_to_file_contribution, FileContribution};

    #[test]
    fn test_file_args_parse_1() {
        let args = "50% f1 50% f2";
        let predicted_result = vec![
            FileContribution {
                file_path: "f1".to_owned(),
                percentage: 50u8,
            },
            FileContribution {
                file_path: "f2".to_owned(),
                percentage: 50u8,
            },
        ];
        assert_eq!(
            file_args_to_file_contribution(args).unwrap(),
            predicted_result
        );
    }

    #[test]
    fn test_file_args_parse2() {
        let args = "50% f1 f2";

        let predicted_result = vec![
            FileContribution {
                file_path: "f1".to_owned(),
                percentage: 50u8,
            },
            FileContribution {
                file_path: "f2".to_owned(),
                percentage: 50u8,
            },
        ];
        assert_eq!(
            file_args_to_file_contribution(args).unwrap(),
            predicted_result
        );
    }

    #[test]
    fn test_file_args_parse3() {
        let args = "75% f1 f2";

        let predicted_result = vec![
            FileContribution {
                file_path: "f1".to_owned(),
                percentage: 75u8,
            },
            FileContribution {
                file_path: "f2".to_owned(),
                percentage: 25u8,
            },
        ];
        assert_eq!(
            file_args_to_file_contribution(args).unwrap(),
            predicted_result
        );
    }

    #[test]
    fn test_file_args_parse4() {
        let args = "f1 f2 f3";

        let predicted_result = vec![
            FileContribution {
                file_path: "f1".to_owned(),
                percentage: 33u8,
            },
            FileContribution {
                file_path: "f2".to_owned(),
                percentage: 33u8,
            },
            FileContribution {
                file_path: "f3".to_owned(),
                percentage: 34u8,
            },
        ];

        assert_eq!(
            file_args_to_file_contribution(args).unwrap(),
            predicted_result
        );
    }
}
