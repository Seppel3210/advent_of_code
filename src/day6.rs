use std::array;

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let [times, distances] = array::from_fn(|_| lines.next().unwrap());
    let prod: f64 = times
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .zip(
            distances
                .trim_start_matches("Distance:")
                .trim()
                .split_whitespace(),
        )
        .map(|(t, d)| {
            let time = t.parse::<f64>().unwrap();
            let distance = d.parse::<f64>().unwrap();

            let speed_min = ((time - (time * time - 4.0 * (distance + 1.0)).sqrt()) / 2.0).ceil();

            let speed_max = ((time + (time * time - 4.0 * (distance + 1.0)).sqrt()) / 2.0).floor();
            speed_max - speed_min + 1.0
        })
        .product();
    format!("{prod}")
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let [time, distance] = array::from_fn(|_| lines.next().unwrap());
    let time = time
        .trim_start_matches("Time:")
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();
    let distance = distance
        .trim_start_matches("Distance:")
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();

    let speed_min = ((time - (time * time - 4.0 * (distance + 1.0)).sqrt()) / 2.0).ceil();

    let speed_max = ((time + (time * time - 4.0 * (distance + 1.0)).sqrt()) / 2.0).floor();
    format!("{}", speed_max - speed_min + 1.0)
}
