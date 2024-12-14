use std::fs;

use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt").into_diagnostic()?;

    let mut list1 = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();

    for line in file.lines() {
        let parts = line.split_once("   ").expect("each line has two parts");

        list1.push(parts.0.parse().expect("each part is a number"));
        list2.push(parts.1.parse().expect("each part is a number"));
    }

    list1.sort();
    list2.sort();

    let mut diff = 0;
    for (&n1, &n2) in list1.iter().zip(list2.iter()) {
        diff += (n1 as i64).abs_diff(n2 as i64);
    }

    println!("{}", diff);

    Ok(())
}
