use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let sum: u64 = input
        .lines()
        .filter_map(|l| {
            let (game_id, game) = l.split_once(':').unwrap();
            let game_id = game_id.trim_start_matches("Game ").parse::<u64>().unwrap();

            let valid = game.split(';').all(|round| {
                round.split(',').all(|cube| {
                    let (count, color) = cube.trim().split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();
                    match color {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        _ => unreachable!(),
                    }
                })
            });
            valid.then_some(game_id)
        })
        .sum();
    format!("{sum}")
}

pub fn part2(input: &str) -> String {
    let sum: u64 = input
        .lines()
        .map(|l| {
            let (_, game) = l.split_once(':').unwrap();
            let mut max_count = HashMap::new();
            for cube in game.split(|c| c == ',' || c == ';') {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let count = count.parse::<u64>().unwrap();
                max_count
                    .entry(color.to_owned())
                    .and_modify(|max: &mut u64| *max = (*max).max(count))
                    .or_insert(count);
            }
            max_count.values().product::<u64>()
        })
        .sum();
    format!("{sum}")
}
