use std::collections::{HashMap, HashSet};

enum Cell {
    Dot,
    NumIndex(usize),
    Sym(char),
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Dot => write!(f, "."),
            Cell::NumIndex(i) => write!(f, "i{i}"),
            Cell::Sym(x) => write!(f, "{x}"),
        }
    }
}

pub fn part1(input: &str) -> String {
    let (numbers, grid) = get_number_grid(input);
    let mut nums_to_sum = HashSet::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line
            .iter()
            .enumerate()
            .filter(|(_, c)| matches!(c, Cell::Sym(_)))
        {
            for y in y.saturating_sub(1)..=y + 1 {
                for x in x.saturating_sub(1)..=x + 1 {
                    if let Some(Cell::NumIndex(num)) = grid.get(y).and_then(|l| l.get(x)) {
                        nums_to_sum.insert(*num);
                    }
                }
            }
        }
    }

    let sum: u32 = nums_to_sum.iter().map(|i| numbers[*i]).sum();
    format!("{sum}")
}

pub fn part2(input: &str) -> String {
    let (numbers, grid) = get_number_grid(input);
    let mut gears = HashMap::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line
            .iter()
            .enumerate()
            .filter(|(_, c)| matches!(c, Cell::Sym('*')))
        {
            for y2 in y.saturating_sub(1)..=y + 1 {
                for x2 in x.saturating_sub(1)..=x + 1 {
                    if let Some(Cell::NumIndex(num)) = grid.get(y2).and_then(|l| l.get(x2)) {
                        gears.entry((x, y)).or_insert(HashSet::new()).insert(*num);
                    }
                }
            }
        }
    }
    let sum: u32 = gears
        .values()
        .map(|indices| {
            if (indices).len() == 2 {
                indices.iter().map(|i| numbers[*i]).product::<u32>()
            } else {
                0
            }
        })
        .sum();
    format!("{sum}")
}

fn get_number_grid(input: &str) -> (Vec<u32>, Vec<Vec<Cell>>) {
    let mut numbers = Vec::new();
    let mut current_num_index = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut current_line = Vec::new();
        for c in line.chars() {
            if let Some(x) = c.to_digit(10) {
                let current_num = if let Some(x) = numbers.get_mut(current_num_index) {
                    x
                } else {
                    numbers.push(0);
                    numbers.last_mut().unwrap()
                };
                *current_num = *current_num * 10 + x;
                current_line.push(Cell::NumIndex(current_num_index));
            } else {
                if current_num_index < numbers.len() {
                    current_num_index += 1;
                }
                if c == '.' {
                    current_line.push(Cell::Dot);
                } else {
                    current_line.push(Cell::Sym(c));
                }
            }
        }
        if current_num_index < numbers.len() {
            current_num_index += 1;
        }
        grid.push(current_line);
    }
    (numbers, grid)
}
