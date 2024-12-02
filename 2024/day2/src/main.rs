const INPUT: &str = include_str!("../input.txt");

fn main() {
    let reports = parse_input();

    let total_reports = reports.len();
    let mut unsafe_reports = 0;

    for report in reports {
        let variants = permutations(report);
        let mut found_safe = false;

        for variant in variants {
            if is_report_safe(variant) {
                found_safe = true;
                break;
            }
        }

        if !found_safe {
            unsafe_reports += 1;
        }
    }

    println!("safe reports: {}", total_reports - unsafe_reports);
}

fn parse_input() -> Vec<Vec<u32>> {
    INPUT
        .split("\n")
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .map(|line| {
            line.split(" ")
                .filter(|s| !s.is_empty())
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: Vec<u32>) -> bool {
    let mut prev_level = 0;
    let mut report_is_increasing = None;

    for level in report {
        if prev_level == 0 {
            prev_level = level;
            continue;
        }

        let delta = level as i32 - prev_level as i32;

        if delta == 0 || delta > 3 || delta < -3 {
            return false;
        }

        let level_is_increasing = if delta > 0 { true } else { false };

        if let Some(report_is_increasing) = report_is_increasing {
            if report_is_increasing != level_is_increasing {
                return false;
            }
        } else {
            report_is_increasing = Some(level_is_increasing);
        }

        prev_level = level;
    }

    true
}

fn permutations(report: Vec<u32>) -> Vec<Vec<u32>> {
    let mut output = vec![];
    output.push(report.clone());

    for i in 0..report.len() {
        let mut copy = vec![];
        for (x, v) in report.iter().enumerate() {
            if x == i {
                continue;
            }
            copy.push(*v);
        }
        output.push(copy);
    }

    output
}
