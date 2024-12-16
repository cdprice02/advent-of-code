use std::collections::HashMap;
use std::fs;

use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt").into_diagnostic()?;

    let mut list_1 = Vec::<u32>::new();
    let mut list_2 = HashMap::<u32, u32>::new();

    for line in file.lines() {
        let parts = line.split_once("   ").expect("each line has two parts");

        list_1.push(parts.0.parse().expect("each part is a number"));

        let num = parts.1.parse().expect("each part is a number");
        *list_2.entry(num).or_insert(0) += 1;
    }

    list_1.sort();

    let diff = list_1
        .iter()
        .fold(0, |acc, &n1| acc + n1 * *list_2.entry(n1).or_insert(0));

    println!("{}", diff);

    Ok(())
}
