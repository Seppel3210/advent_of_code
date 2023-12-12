use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(':').unwrap();
            let (winning_nums, nums) = card.split_once('|').unwrap();

            let winning_nums = winning_nums
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let winners = nums
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .filter(|n| winning_nums.contains(n))
                .count();
            if winners == 0 {
                0
            } else {
                1 << (winners - 1)
            }
        })
        .sum();
    format!("{sum}")
}

pub fn part2(input: &str) -> String {
    let mut card_amounts = Vec::new();
    let mut line_count = 0;
    for (i, line) in input.lines().enumerate() {
        line_count += 1;
        let (_, card) = line.split_once(':').unwrap();
        let (winning_nums, nums) = card.split_once('|').unwrap();

        let winning_nums = winning_nums
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let winners = nums
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .filter(|n| winning_nums.contains(n))
            .count();
        let current_amount = card_amounts.get(i).copied().unwrap_or_else(|| {
            card_amounts.push(1);
            1
        });
        for j in 0..winners {
            let amount = match card_amounts.get_mut(i + j + 1) {
                Some(x) => x,
                None => {
                    card_amounts.push(1);
                    card_amounts.last_mut().unwrap()
                }
            };
            *amount += current_amount;
        }
    }

    let sum: u32 = card_amounts[..line_count].iter().sum();
    format!("{sum}")
}
