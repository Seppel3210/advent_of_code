use std::iter;

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn sequences(seq: &[i64]) -> Vec<Vec<i64>> {
    let seqs = iter::successors(Some(seq.to_owned()), |seq| {
        let mut all_zero = seq[0] == 0;
        let new_seq = seq
            .windows(2)
            .map(|w| {
                all_zero &= w[1] == 0;
                w[1] - w[0]
            })
            .collect();
        (!all_zero).then_some(new_seq)
    })
    .collect::<Vec<_>>();
    seqs
}

pub fn part1(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(|l| {
            let start_seq = parse_line(l);
            let seqs = sequences(&start_seq);
            let seq_sum: i64 = seqs.iter().map(|s| s.last().unwrap()).sum();
            seq_sum
        })
        .sum();
    format!("{sum}")
}

pub fn part2(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(|l| {
            let start_seq = parse_line(l);
            let seqs = sequences(&start_seq);
            let seq_sum: i64 = seqs
                .iter()
                .zip([1, -1].into_iter().cycle())
                .map(|(s, i)| i * s.first().unwrap())
                .sum();
            seq_sum
        })
        .sum();
    format!("{sum}")
}
