use std::{env, fs::read_to_string};

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

    for report in reports {
        if is_report_safe(report) {
            safe_reports_count += 1;
        }
    }

    safe_reports_count
}

fn is_report_safe(levels: Vec<i32>) -> bool {
    let mut should_increase = true;

    // report with single level - safe
    if levels.len() == 1 {
        return true;
    }

    let diff = levels[1] - levels[0];

    // first two levels are the same or increased/decreased by more than 3 - not safe
    if diff == 0 || diff.abs() > 3 {
        return false;
    }

    if diff < 0 {
        should_increase = false;
    }

    // already checked first two results, zipping iterators with skip(1) and skip(2)
    let levels_iter = levels.iter().skip(1).zip(levels.iter().skip(2));

    for (prev_level, next_level) in levels_iter {
        let diff = next_level - prev_level;

        // equal levels, not safe
        if diff == 0 {
            return false;
        }

        // should increase, but not increase, not safe
        if should_increase && diff < 0 {
            return false;
        }

        // should decrease, but increased, not safe
        if !should_increase && diff > 0 {
            return false;
        }

        // increased or decreased by more than 3, not safe
        if diff.abs() > 3 {
            return false;
        }
    }

    // if reached this point, report is safe
    return true;
}
