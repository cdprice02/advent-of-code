use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input.txt").into_diagnostic()?;
    let chars = file.chars().collect::<Vec<_>>();

    let mut muls = Vec::<(u32, u32)>::new();
    let mut i = 0;
    while i < chars.len() - 4 {
        if chars[i..i + 4] == ['m', 'u', 'l', '('] {
            i += 4;

            let a = if let Some((a, len)) = read_first_num(&chars[i..]) {
                i += len;
                a
            } else {
                i += 1;
                continue;
            };

            i += 1;
            if chars[i - 1] != ',' {
                continue;
            }

            let b = if let Some((b, len)) = read_first_num(&chars[i..]) {
                i += len;
                b
            } else {
                i += 1;
                continue;
            };

            if chars[i] == ')' {
                muls.push((a, b));
            }
        }

        i += 1;
    }

    let score = muls.iter().fold(0, |acc, (a, b)| acc + a * b);
    println!("{}", score); // 169021493

    Ok(())
}

fn read_first_num(chars: &[char]) -> Option<(u32, usize)> {
    let mut s = String::new();
    let mut i = 0;
    while i < chars.len() && chars[i].is_ascii_digit() {
        s.push(chars[i]);
        i += 1;
    }
    s.parse().ok().map(|n| (n, i))
}
