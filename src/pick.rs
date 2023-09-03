use std::process::exit;

use crate::file_utils::{self, FileContribution, FortuneResult};

pub fn pick_line_from_files_uniform(files: Vec<String>) -> Result<FortuneResult, String> {
    let mut fortunes = Vec::new();
    for file in &files {
        match file_utils::get_fortunes_from_file(&file) {
            Ok(e) => fortunes.extend(e),
            Err(error) => {
                return Err(error);
            }
        }
    }
    if files.len() == 0 {
        return Err(String::from("No fortune files found"));
    }
    return Ok(fortunes[fastrand::usize(0..fortunes.len())].clone());
}

pub fn pick_line_from_file_contributions(contributions: Vec<FileContribution>) -> FortuneResult {
    let mut pick_array: [u8; 100] = [0; 100];
    let mut cur_index = 0;
    for i in 0..contributions.len() {
        for j in cur_index..(cur_index + contributions[i].percentage) {
            pick_array[j as usize] = i as u8;
            cur_index += 1;
        }
    }
    let pick = pick_array[fastrand::usize(0..100)];
    match file_utils::get_fortunes_from_file(&contributions[pick as usize].file_path) {
        Ok(e) => return e[fastrand::usize(0..e.len())].clone(),
        Err(error) => println!("{error}"),
    }
    return FortuneResult {
        fortune: String::from(""),
        file_path: String::from(""),
    };
}
