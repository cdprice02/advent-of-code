use std::fs;

use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt").into_diagnostic()?;

    let mut list_1 = Vec::<u32>::new();
    let mut list_2 = Vec::<u32>::new();

    for line in file.lines() {
        let parts = line.split_once("   ").expect("each line has two parts");

        list_1.push(parts.0.parse().expect("each part is a number"));
        list_2.push(parts.1.parse().expect("each part is a number"));
    }

    list_1.sort();
    list_2.sort();

    let diff = list_1
        .iter()
        .zip(list_2.iter())
        .fold(0, |acc, (&n1, &n2)| acc + (n1 as i64).abs_diff(n2 as i64));

    println!("{}", diff);

    Ok(())
}
