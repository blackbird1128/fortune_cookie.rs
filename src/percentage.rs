use crate::file_utils::FileContribution;

pub fn fill_contributions(contributions: &mut Vec<FileContribution>) {
    let len = contributions.len();
    let unset_count = contributions.iter().filter(|x| x.percentage == 0).count();
    let percentage_sum = contributions.iter().fold(0, |mut acc, x| {
        acc += x.percentage;
        acc
    });
    if percentage_sum > 100 {
        panic!("Percents in the file contribution vec sum up to x > 100 ");
    }
    let to_fill = unset_count;
    let rest: i32 = (100 - percentage_sum) as i32;
    let mut fill = 0;
    if rest % to_fill as i32 != 0 {
        fill += 1
    }
    let fill = rest / to_fill as i32 + fill;
    contributions
        .iter_mut()
        .filter(|x| x.percentage == 0)
        .for_each(|x| x.percentage = fill as u8);
}

#[cfg(test)]
mod tests {
    use crate::file_utils::FileContribution;
    use crate::percentage::fill_contributions;

    #[test]
    fn test_fill_contributions() {
        let mut contributions = vec![
            FileContribution {
                file_path: "f1".to_owned(),
                percentage: 0,
            },
            FileContribution {
                file_path: "f2".to_owned(),
                percentage: 0,
            },
        ];
        fill_contributions(&mut contributions);
        assert_eq!(contributions[0].percentage, 50);
        assert_eq!(contributions[1].percentage, 50);
    }
}
