use std::collections::HashMap;

fn solutions<'a, 'b>(
    line: &'a str,
    groups: &'b [u32],
    cache: &mut HashMap<(&'a str, &'b [u32]), u64>,
) -> u64 {
    if let Some(cached) = cache.get(&(line, groups)) {
        return *cached;
    }
    let Some(group) = groups.first() else {
        return if line.chars().any(|c| c == '#') { 0 } else { 1 };
    };
    let mut sum = 0;
    if !line
        .get(..*group as usize)
        .unwrap_or(".")
        .chars()
        .any(|c| c == '.')
    {
        if let Some(c) = line.chars().nth(*group as usize) {
            if c != '#' {
                sum += solutions(&line[(*group + 1) as usize..], &groups[1..], cache);
            }
        } else {
            if groups.len() == 1 {
                sum += 1;
            }
        }
    }
    if line.chars().next().unwrap_or('#') != '#' {
        sum += solutions(&line[1..], groups, cache);
    }
    cache.insert((line, groups), sum);
    sum
}

pub fn part1(input: &str) -> String {
    let sum: u64 = input
        .lines()
        .map(|l| {
            let mut cache = HashMap::new();
            let (line, groups) = l.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .map(|g| g.parse().unwrap())
                .collect::<Vec<_>>();
            let solutions = solutions(line, &groups, &mut cache);
            solutions
        })
        .sum();
    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let sum: u64 = input
        .lines()
        .map(|l| {
            let mut cache = HashMap::new();
            let (line, groups) = l.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .map(|g| g.parse().unwrap())
                .collect::<Vec<_>>();
            let line = [line].repeat(5).join("?");
            let groups = groups.repeat(5);
            let solutions = solutions(&line, &groups, &mut cache);
            solutions
        })
        .sum();
    sum.to_string()
}
