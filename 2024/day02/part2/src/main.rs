use std::{env, fs::read_to_string, usize};

use ordermap::OrderSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let reports = read_reports(file_path.to_string());

    println!("{}", count_safe_reports(reports));
}

fn read_reports(file_path: String) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        let report = line
            .split(' ')
            .into_iter()
            .map(|level| level.parse::<i32>().unwrap())
            .collect();

        reports.push(report);
    }

    reports
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports_count = 0;

    'reports: for report in reports {
        let incorrect_level_indexes = get_incorrect_level_indexes(&report);

        // safe report - moving on
        if incorrect_level_indexes.len() == 0 {
            safe_reports_count += 1;
            continue;
        }

        // try removing incorrect levels and check if it makes report safe
        for index in incorrect_level_indexes {
            let mut report_without_incorrect_level = report.clone();
            report_without_incorrect_level.remove(index);

            let incorrect_with_removed =
                get_incorrect_level_indexes(&report_without_incorrect_level);

            // report became safe with removed level - move on
            if incorrect_with_removed.len() == 0 {
                safe_reports_count += 1;
                continue 'reports;
            }
        }
    }

    safe_reports_count
}

fn get_incorrect_level_indexes(levels: &Vec<i32>) -> OrderSet<usize> {
    let mut should_increase = true;
    let mut incorrect_level_indexes = OrderSet::new();

    // report with single level - safe
    if levels.len() == 1 {
        return incorrect_level_indexes;
    }

    let diff = levels[1] - levels[0];

    // first two levels are the same, add the first one as incorrect
    if diff == 0 {
        incorrect_level_indexes.insert(0);
    }

    // first two levels increased/decreased by more than 3, either one can be later removed
    if diff.abs() > 3 {
        incorrect_level_indexes.insert(0);
        incorrect_level_indexes.insert(1);
    }

    if diff < 0 {
        should_increase = false;
    }

    if levels.len() > 2 {
        let diff_with_third = levels[2] - levels[1];

        // first two levels indicate that levels should increase, but the next one decreases, maybe
        // removing the first or second level makes all values decreasing
        if should_increase && diff_with_third < 0 {
            incorrect_level_indexes.insert(0);
            incorrect_level_indexes.insert(1);
        }

        // first two levels indicate that levels should decrease, but the next one increases,
        // maybe removing first or second level makes report safe
        if !should_increase && diff_with_third > 0 {
            incorrect_level_indexes.insert(0);
            incorrect_level_indexes.insert(1);
        }
    }

    // already checked first two levels, zipping iterators with skip(1) and skip(2)
    let levels_iter = levels
        .iter()
        .enumerate()
        .skip(1)
        .zip(levels.iter().enumerate().skip(2));

    for ((prev_index, prev_level), (next_index, next_level)) in levels_iter {
        let diff = next_level - prev_level;

        // equal levels, not safe
        if diff == 0 {
            incorrect_level_indexes.insert(prev_index);
            incorrect_level_indexes.insert(next_index);
        }

        // should increase, but not increased, not safe
        if should_increase && diff < 0 {
            incorrect_level_indexes.insert(prev_index);
            incorrect_level_indexes.insert(next_index);
        }

        // should decrease, but increased, not safe
        if !should_increase && diff > 0 {
            incorrect_level_indexes.insert(prev_index);
            incorrect_level_indexes.insert(next_index);
        }

        // increased or decreased by more than 3, not safe
        if diff.abs() > 3 {
            incorrect_level_indexes.insert(prev_index);
            incorrect_level_indexes.insert(next_index);
        }
    }

    // if reached this point, report is safe
    incorrect_level_indexes
}
