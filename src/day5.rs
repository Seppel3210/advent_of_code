use std::{
    array,
    collections::{btree_map::Cursor, BTreeMap},
    ops::Bound,
};

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut maps = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            maps.push(BTreeMap::new());
            lines.next();
            continue;
        }
        let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        let [dst, src, len] = array::from_fn(|_| nums.next().unwrap());
        let map = maps.last_mut().unwrap();
        map.insert(src, (dst, len));
    }

    for map in &maps {
        for seed in &mut seeds {
            let Some((&src, &(dst, len))) = map.upper_bound(Bound::Included(seed)).peek_prev()
            else {
                continue;
            };
            let offset = *seed - src;
            if offset < len {
                *seed = dst + offset;
            };
        }
    }

    format!("{}", seeds.iter().min().unwrap())
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut seed_ranges = seeds.chunks(2).map(|s| (s[0], s[1])).collect::<Vec<_>>();
    let mut maps = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            maps.push(BTreeMap::new());
            lines.next();
            continue;
        }
        let mut nums = line.split_whitespace().map(|n| n.parse::<u64>().unwrap());
        let [dst, src, len] = array::from_fn(|_| nums.next().unwrap());
        let map = maps.last_mut().unwrap();
        map.insert(src, (dst, len));
    }

    let mut current_ranges = Vec::new();
    for map in &maps {
        current_ranges.append(&mut seed_ranges);
        while let Some((start, len)) = current_ranges.pop() {
            if len == 0 {
                continue;
            }
            let cursor = map.upper_bound(Bound::Included(&start));
            if let Some((&src, &(dst, map_len))) = cursor.peek_prev() {
                if src + map_len <= start {
                    check_next_map(cursor, start, len, &mut seed_ranges, &mut current_ranges);
                } else {
                    let offset = start - src;
                    let mapped_count = src + map_len - start;
                    seed_ranges.push((dst + offset, mapped_count.min(len)));
                    current_ranges.push((start + mapped_count, len.saturating_sub(mapped_count)));
                }
            } else {
                check_next_map(cursor, start, len, &mut seed_ranges, &mut current_ranges);
            }
        }
    }

    format!(
        "{}",
        seed_ranges
            .iter()
            .map(|(range_start, _)| range_start)
            .min()
            .unwrap()
    )
}

fn check_next_map(
    mut cursor: Cursor<u64, (u64, u64)>,
    start: u64,
    len: u64,
    seed_ranges: &mut Vec<(u64, u64)>,
    current_ranges: &mut Vec<(u64, u64)>,
) {
    let Some((&src, _)) = cursor.next() else {
        seed_ranges.push((start, len));
        return;
    };
    if start + len <= src {
        seed_ranges.push((start, len));
    } else {
        let mapped_count = start + len - src;
        seed_ranges.push((start, len - mapped_count));
        current_ranges.push((start + mapped_count, mapped_count));
    }
}
