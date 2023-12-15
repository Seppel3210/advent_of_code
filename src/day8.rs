use std::collections::HashMap;
pub fn part1(input: &str) -> String {
    let (sequence, nodes) = parse_nodes(input).unwrap();
    format!("{}", steps_until_suffix(sequence, &nodes, "AAA", "ZZZ"))
}

pub fn part2(input: &str) -> String {
    let (sequence, nodes) = parse_nodes(input).unwrap();
    let mut steps = 1;
    for start in nodes.keys().filter(|n| n.ends_with('A')) {
        let current_steps = steps_until_suffix(sequence, &nodes, start, "Z");
        steps = lcm(steps, current_steps);
    }
    format!("{steps}")
}

fn parse_nodes(input: &str) -> Option<(&str, HashMap<&str, (&str, &str)>)> {
    let mut lines = input.lines();
    let sequence = lines.next()?;
    lines.next()?;

    let mut nodes = HashMap::new();
    for line in lines {
        let (name, adjacent) = line.split_once('=')?;
        let (l, r) = adjacent.split_once(',')?;
        nodes.insert(
            name.trim(),
            (l.strip_prefix(" (")?, r.trim().strip_suffix(')')?),
        );
    }
    Some((sequence, nodes))
}

fn steps_until_suffix(
    sequence: &str,
    nodes: &HashMap<&str, (&str, &str)>,
    start_node: &str,
    suffix: &str,
) -> u64 {
    let mut current_node = start_node;
    let mut steps = 0;
    let mut seq_state = sequence.chars().cycle();
    while !current_node.ends_with(suffix) {
        let (l, r) = nodes[current_node];
        current_node = match seq_state.next().unwrap() {
            'L' => l,
            'R' => r,
            _ => unimplemented!(),
        };
        steps += 1;
    }
    steps
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}
