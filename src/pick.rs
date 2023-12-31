use std::process::exit;

use crate::file_utils::{self, FileContribution, FortuneResult};

pub struct FortuneFilter {
    pub file: FilterFile,
    pub len: FilterLen,
    pub len_value: u32,
}

pub enum FilterFile {
    Default,
    All,
    Offensive,
}

pub enum FilterLen {
    Short, // < len
    Long,  // > len
}

pub fn filter_fortunes(fortunes: Vec<FortuneResult>, filter: FortuneFilter) -> Vec<FortuneResult> {
    fortunes
        .iter()
        .filter(|x| {
            let len = x.fortune.len() as u32;
            match filter.len {
                FilterLen::Short => len < filter.len_value,
                FilterLen::Long => len > filter.len_value,
            }
        })
        .filter(|x| match filter.file {
            FilterFile::Default => !x.file_path.contains("off"),
            FilterFile::All => true,
            FilterFile::Offensive => x.file_path.contains("off"),
        })
        .map(|x| x.clone())
        .collect()
}

pub fn pick_all_from_files(files: Vec<String>) -> Result<Vec<FortuneResult>, String> {
    let mut fortunes = Vec::new();
    for file in &files {
        if let Ok(e) = file_utils::get_fortunes_from(&file) {
            fortunes.extend(e)
        } else {
            continue;
        }
    }
    Ok(fortunes)
}

pub fn pick_line_from_files_uniform(
    files: Vec<String>,
    filter: FortuneFilter,
) -> Result<FortuneResult, String> {
    let mut fortunes = Vec::new();
    for file in &files {
        if let Ok(e) = file_utils::get_fortunes_from(&file) {
            fortunes.extend(e)
        } else {
            continue;
        }
    }
    if files.len() == 0 {
        return Err(String::from("No fortune files with these names found"));
    }
    let fortunes = filter_fortunes(fortunes, filter);
    if fortunes.len() == 0 {
        return Err(String::from("No fortunes satisfying filter(s) found"));
    }

    // TODO: add len filtering
    return Ok(fortunes[fastrand::usize(0..fortunes.len())].clone());
}

pub fn pick_line_from_file_contributions(
    contributions: Vec<FileContribution>,
    filter: FortuneFilter,
) -> FortuneResult {
    let mut pick_array: [u8; 100] = [0; 100];
    let mut cur_index = 0;
    for i in 0..contributions.len() {
        for j in cur_index..(cur_index + contributions[i].percentage) {
            pick_array[j as usize] = i as u8;
            cur_index += 1;
        }
    }
    let pick = pick_array[fastrand::usize(0..100)];
    let fortunes = file_utils::get_fortunes_from(&contributions[pick as usize].path);
    let fortunes = fortunes.unwrap_or_else(|e| {
        eprintln!("Error: {e} ");
        exit(1);
    });
    let fortunes = filter_fortunes(fortunes, filter);
    if fortunes.len() == 0 {
        eprintln!("Error: no fortunes satisfying filter(s) found");
        exit(1);
    }
    return fortunes[fastrand::usize(0..fortunes.len())].clone();
}
