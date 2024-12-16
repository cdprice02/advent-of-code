use std::fs;

use miette::{IntoDiagnostic, Result};

type Floor = i32;

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt").into_diagnostic()?;

    let mut num_safe_reports = 0;

    for line in file.lines() {
        let floors = line
            .split(' ')
            .map(|x| x.parse::<Floor>().expect("all floors are numbers"))
            .collect::<Vec<_>>();

        // check without removal
        if check_report(floors.clone()) {
            num_safe_reports += 1;
            continue;
        }

        // check all possible single removals
        for remove_idx in 0..floors.len() {
            let mut floors = floors.clone();
            floors.remove(remove_idx);
            if check_report(floors) {
                num_safe_reports += 1;
                break;
            }
        }
    }

    println!("{}", num_safe_reports);

    Ok(())
}

fn check_report(floors: Vec<Floor>) -> bool {
    assert!(floors.len() >= 2, "level should have at least 2 floors");

    let init_diff = floor_diff(floors[0], floors[1]);
    if !valid_diff(init_diff) {
        return false;
    }

    for i in 1..floors.len() - 1 {
        let diff = floor_diff(floors[i], floors[i + 1]);
        if !check_diff(init_diff, diff) {
            return false;
        }
    }
    true
}

fn floor_diff(floor_1: Floor, floor_2: Floor) -> i32 {
    floor_2 - floor_1
}

fn check_diff(init_diff: i32, diff: i32) -> bool {
    valid_diff(diff) && diff.signum() == init_diff.signum()
}

fn valid_diff(diff: i32) -> bool {
    diff.abs() >= 1 && diff.abs() <= 3
}
