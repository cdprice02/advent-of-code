use miette::{IntoDiagnostic, Result};
use regex::Regex;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input.txt").into_diagnostic()?;

    let mut sections = file.split("don't()");

    let init_muls = get_mul_instructions(
        sections
            .next()
            .expect("split results in at least one section"),
    );
    let mut total_score = init_muls.iter().fold(0, |acc, (a, b)| acc + a * b);

    for section in sections {
        if let Some((_disabled_section, enabled_section)) = section.split_once("do()") {
            let muls = get_mul_instructions(enabled_section);
            let score = muls.iter().fold(0, |acc, (a, b)| acc + a * b);

            total_score += score;
        }
    }

    println!("{}", total_score);

    Ok(())
}

fn get_mul_instructions(s: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    re.captures_iter(s)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (
                a.parse::<u32>().expect("all captures should be numbers"),
                b.parse::<u32>().expect("all captures should be numbers"),
            )
        })
        .collect::<Vec<_>>()
}
