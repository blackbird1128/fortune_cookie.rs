use crate::file_utils::FileContribution;

pub fn fill_contributions(contributions: &mut Vec<FileContribution>) -> Result<(), String> {
    let len = contributions.len();
    let unset_count = contributions.iter().filter(|x| x.percentage == 0).count();
    let percentage_sum = contributions.iter().fold(0, |mut acc, x| {
        acc += x.percentage;
        acc
    });
    if percentage_sum > 100 {
        return Err(String::from(
            "Percents in the file contribution vec sum up to x > 100",
        ));
    }
    if unset_count == 0 && percentage_sum == 100 {
        return Ok(());
    } else if unset_count == 0 && percentage_sum != 100 {
        return Err(String::from(
            "Percents in the file contribution vec sum up to x != 100 : Not fixable",
        ));
    }
    let percentage_left = 100 - percentage_sum;

    if unset_count != 0 {
        let rest = percentage_left % unset_count as u8;
        for i in 0..len {
            if contributions[i].percentage == 0 {
                contributions[i].percentage = (100 - percentage_sum) / unset_count as u8;
                if i == len - 1 {
                    contributions[i].percentage += rest;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::file_utils::FileContribution;
    use crate::percentage::fill_contributions;

    #[test]
    fn test_fill_contributions1() {
        let mut contributions = vec![
            FileContribution {
                path: "f1".to_owned(),
                percentage: 0,
            },
            FileContribution {
                path: "f2".to_owned(),
                percentage: 0,
            },
        ];

        let _ = fill_contributions(&mut contributions);
        assert_eq!(contributions[0].percentage, 50);
        assert_eq!(contributions[1].percentage, 50);
    }

    #[test]
    fn test_fill_contributions2() {
        let mut contributions = vec![
            FileContribution {
                path: "f1".to_owned(),
                percentage: 0,
            },
            FileContribution {
                path: "f2".to_owned(),
                percentage: 0,
            },
            FileContribution {
                path: "f3".to_owned(),
                percentage: 0,
            },
        ];

        let _ = fill_contributions(&mut contributions);
        assert_eq!(contributions[0].percentage, 33);
        assert_eq!(contributions[1].percentage, 33);
        assert_eq!(contributions[2].percentage, 34);
    }

    #[test]
    fn test_fill_contributions3() {
        let mut contributions = vec![
            FileContribution {
                path: "f1".to_owned(),
                percentage: 0,
            },
            FileContribution {
                path: "f2".to_owned(),
                percentage: 98,
            },
        ];

        let _ = fill_contributions(&mut contributions);
        assert_eq!(contributions[0].percentage, 2);
        assert_eq!(contributions[1].percentage, 98);
    }
}
