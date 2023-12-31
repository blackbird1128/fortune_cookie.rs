use std::fs;
use std::fs::metadata;
use std::process::exit;

use regex::Regex;
use walkdir::WalkDir;

use crate::{fortune, percentage};

#[derive(Debug, PartialEq)]
pub struct FileContribution {
    pub path: String,
    pub percentage: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FortuneResult {
    pub fortune: String,
    pub file_path: String,
}

pub fn get_fortune_files(vec_folders: &[&str]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for folder in vec_folders {
        for file in WalkDir::new(folder)
            .into_iter()
            .filter_map(|file| file.ok())
        {
            let metadata = file.metadata().unwrap_or_else(|e| {
                eprintln!("Error: could not get metadata for file: {}", e);
                exit(1);
            });
            if metadata.is_file() {
                let file_path = file.path().to_str().unwrap_or_else(|| {
                    eprintln!("Error converting file path (invalid utf-8)");
                    exit(1);
                });
                let file_path = file_path.to_owned();
                if !files.contains(&file_path) {
                    files.push(file_path);
                }
                continue;
            }

            if metadata.is_dir() {
                files.extend(expand_folder_into_files(file.path().to_str().unwrap()));
                continue;
            }
        }
    }
    files
}

pub fn expand_folder_into_files(folder: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for file in WalkDir::new(folder)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        let metadata = file.metadata().unwrap_or_else(|e| {
            eprintln!("Error: could not get metadata for file: {}", e);
            exit(1);
        });
        if metadata.is_file() {
            files.push(file.path().to_str().unwrap().to_owned());
        }
    }
    files
}

pub fn get_fortunes_from(path: &str) -> Result<Vec<FortuneResult>, String> {
    // check if the path is a folder: if true: get all the files in the folder and pick a fortune
    // from one of them
    let metadata = metadata(path);
    if metadata.is_err() {
        return Err(format!("File not found: {}", path));
    }
    let metadata = metadata.unwrap();
    if metadata.is_dir() {
        let files = get_fortune_files(&[path]);
        let mut fortunes: Vec<FortuneResult> = Vec::new();
        for file in files {
            let file_fortunes = get_fortunes_from(&file);
            match file_fortunes {
                Ok(x) => fortunes.extend(x),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        return Ok(fortunes);
    } else {
        let file_contents = fs::read_to_string(path);
        if let Ok(x) = file_contents {
            Ok(fortune::parse_fortune_string(&x, &path))
        } else {
            Err(format!("Error: could not read file {}", path))
        }
    }
}

pub fn file_args_to_file_contribution(args: &str) -> Result<Vec<FileContribution>, String> {
    let re = Regex::new(r"(\d\d?%?\s)?(\S+)+").unwrap();
    let mut file_structs: Vec<FileContribution> = Vec::new();
    re.captures_iter(&args).for_each(|cap| {
        let mut cur_struct = FileContribution {
            percentage: 0,
            path: cap.get(2).unwrap().as_str().to_owned(),
        };
        let path_metadata = metadata(&cur_struct.path);
        if path_metadata.is_err() {
            // if the path doesnt exist, we try appending it to the default paths
            for folder in crate::conf::DEFAULT_FOLDERS.iter() {
                let path = format!("{}/{}", folder, &cur_struct.path);
                let path_metadata = metadata(&path);
                if path_metadata.is_err() {
                    continue;
                }
                if path_metadata.is_ok() {
                    cur_struct.path = path;
                    break;
                }
            }
        }
        cur_struct.percentage = match cap.get(1) {
            Some(x) => {
                let x = x.as_str().trim().replace("%", "");
                x.as_str().trim().parse().unwrap_or(0)
            }
            None => 0,
        };
        file_structs.push(cur_struct);
    });
    if let Err(x) = percentage::fill_contributions(&mut file_structs) {
        return Err(x);
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
                path: "f1".to_owned(),
                percentage: 50u8,
            },
            FileContribution {
                path: "f2".to_owned(),
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
                path: "f1".to_owned(),
                percentage: 50u8,
            },
            FileContribution {
                path: "f2".to_owned(),
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
                path: "f1".to_owned(),
                percentage: 75u8,
            },
            FileContribution {
                path: "f2".to_owned(),
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
                path: "f1".to_owned(),
                percentage: 33u8,
            },
            FileContribution {
                path: "f2".to_owned(),
                percentage: 33u8,
            },
            FileContribution {
                path: "f3".to_owned(),
                percentage: 34u8,
            },
        ];

        assert_eq!(
            file_args_to_file_contribution(args).unwrap(),
            predicted_result
        );
    }
}
