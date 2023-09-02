use crate::file_utils::{self, FileContribution};

pub fn pick_line_from_files_uniform(files: Vec<String>) -> String {
    let mut fortunes = Vec::new();
    for file in files {
        match file_utils::get_fortunes_from_file(&file) {
            Ok(e) => fortunes.extend(e),
            Err(error) => println!("{error}"),
        }
    }
    return fortunes[fastrand::usize(0..fortunes.len())].clone();
}

pub fn pick_line_from_file_contributions(contributions: Vec<FileContribution>) -> String {
    return "TODO".to_owned();
}
