use std::fs;

use miette::{IntoDiagnostic, Result};

type Room = i32;

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt").into_diagnostic()?;

    let mut num_safe_floors = 0;

    'line: for line in file.lines() {
        let floor = line
            .split(' ')
            .map(|x| x.parse::<Room>().expect("all rooms are numbers"))
            .collect::<Vec<_>>();

        assert!(floor.len() >= 2, "floor should have at least 2 rooms");

        let init_diff = room_diff(floor[0], floor[1]);
        if !valid_diff(init_diff) {
            continue;
        }

        for i in 1..floor.len() - 1 {
            let diff = room_diff(floor[i], floor[i + 1]);
            if !check_diff(init_diff, diff) {
                continue 'line;
            }
        }

        num_safe_floors += 1;
    }

    println!("{}", num_safe_floors);

    Ok(())
}

fn room_diff(room_1: Room, room_2: Room) -> i32 {
    room_2 - room_1
}

fn check_diff(init_diff: i32, diff: i32) -> bool {
    valid_diff(diff) && diff.signum() == init_diff.signum()
}

fn valid_diff(diff: i32) -> bool {
    diff.abs() >= 1 && diff.abs() <= 3
}
