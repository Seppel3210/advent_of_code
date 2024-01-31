use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let galaxies = parse_input(input, 2);
    let sum = pairwise_sum(&galaxies);
    format!("{sum}")
}

pub fn part2(input: &str) -> String {
    let galaxies = parse_input(input, 1_000_000);
    let sum = pairwise_sum(&galaxies);
    format!("{sum}")
}

fn parse_input(input: &str, multiplicity: usize) -> Vec<(usize, usize)> {
    let row_len = input.find('\n').unwrap();
    let col_len = input.len() / (row_len + 1);
    let mut row_empty = vec![true; row_len];
    let mut col_empty = vec![true; col_len + 1];
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                row_empty[y] = false;
                col_empty[x] = false;
                galaxies.push((x, y));
            }
        }
    }
    let mut x_offset = 0;
    for x in col_empty
        .iter()
        .enumerate()
        .filter_map(|(x, b)| b.then_some(x))
    {
        for (gx, _) in galaxies.iter_mut() {
            if *gx > x + x_offset {
                *gx += multiplicity - 1;
            }
        }
        x_offset += multiplicity - 1;
    }
    let mut y_offset = 0;
    for y in row_empty
        .iter()
        .enumerate()
        .filter_map(|(y, b)| b.then_some(y))
    {
        for (_, gy) in galaxies.iter_mut() {
            if *gy > y + y_offset {
                *gy += multiplicity - 1;
            }
        }
        y_offset += multiplicity - 1;
    }
    galaxies
}

#[allow(unused)]
fn print_galaxy(galaxy: &[(usize, usize)]) {
    let galaxy: HashSet<_> = galaxy.iter().copied().collect();
    let &(_, rows) = galaxy.iter().max_by_key(|(_, y)| y).unwrap();
    let &(cols, _) = galaxy.iter().max_by_key(|(x, _)| x).unwrap();
    for y in 0..=rows {
        for x in 0..=cols {
            if galaxy.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn pairwise_sum(galaxies: &[(usize, usize)]) -> usize {
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let distance =
                galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
            sum += distance;
        }
    }
    sum
}
