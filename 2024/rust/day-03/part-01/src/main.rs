use miette::{IntoDiagnostic, Result};
use regex::Regex;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input.txt").into_diagnostic()?;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let muls = re
        .captures_iter(&file)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (
                a.parse::<u32>().expect("all captures should be numbers"),
                b.parse::<u32>().expect("all captures should be numbers"),
            )
        })
        .collect::<Vec<_>>();

    let score = muls.iter().fold(0, |acc, (a, b)| acc + a * b);
    println!("{}", score); // 169021493

    Ok(())
}
