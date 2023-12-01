pub fn part1(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|l| {
            let first = l.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = l.chars().rfind(|c| c.is_ascii_digit()).unwrap();
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum();
    format!("{sum}")
}

fn parse_line(line: &str) -> (u32, u32) {
    let digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut first = 0;
    'outer: for i in 0..line.len() {
        if let Some(val) = line[i..].chars().next().unwrap().to_digit(10) {
            first = val;
            break 'outer;
        } else {
            for &(name, val) in &digits {
                if line[i..].starts_with(name) {
                    first = val;
                    break 'outer;
                }
            }
        }
    }

    let mut last = 0;
    'outer: for i in (0..=line.len()).rev() {
        if let Some(val) = line[..i].chars().last().unwrap().to_digit(10) {
            last = val;
            break 'outer;
        } else {
            for &(name, val) in &digits {
                if line[..i].ends_with(name) {
                    last = val;
                    break 'outer;
                }
            }
        }
    }
    (first, last)
}

pub fn part2(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|l| {
            let (first, last) = parse_line(l);
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum();
    format!("{sum}")
}
