use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Connection {
    Start,
    North,
    East,
    South,
    West,
    Empty,
}

impl Connection {
    fn reverse(self) -> Self {
        use Connection as C;
        match self {
            C::Start => C::Start,
            C::North => C::South,
            C::East => C::West,
            C::South => C::North,
            C::West => C::East,
            C::Empty => C::Empty,
        }
    }
    fn to_offset(self) -> (isize, isize) {
        match self {
            Connection::North => (0, -1),
            Connection::East => (1, 0),
            Connection::South => (0, 1),
            Connection::West => (-1, 0),
            Connection::Empty | Connection::Start => (0, 0),
        }
    }

    fn offset(self, x: usize, y: usize) -> (usize, usize) {
        let (x_off, y_off) = self.to_offset();
        ((x as isize + x_off) as usize, (y as isize + y_off) as usize)
    }
}

type Tile = [Connection; 2];

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    use Connection as C;
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => [C::Empty, C::Empty],
                    'S' => [C::Start, C::Start],
                    '|' => [C::North, C::South],
                    '-' => [C::East, C::West],
                    'L' => [C::North, C::East],
                    'J' => [C::North, C::West],
                    '7' => [C::South, C::West],
                    'F' => [C::South, C::East],
                    _ => [C::Empty, C::Empty],
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let map = parse_input(input);
    let (start_x, start_y, _) = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, con)| (x, y, con)))
        .find(|(_, _, con)| con.contains(&Connection::Start))
        .unwrap();
    let mut visit_next = [
        Connection::North,
        Connection::East,
        Connection::South,
        Connection::West,
    ]
    .into_iter()
    .filter_map(|con| {
        let (x, y) = con.offset(start_x, start_y);
        (map[y][x].contains(&con.reverse())).then_some((x, y))
    })
    .collect::<VecDeque<_>>();
    let mut visited = HashSet::from([(start_x, start_y)]);
    let mut distance = 0;
    while let Some(points) = visit_next
        .pop_front()
        .and_then(|a| Some([a, visit_next.pop_front()?]))
    {
        for &(x, y) in &points {
            visit_next.extend(
                map[y][x]
                    .into_iter()
                    .map(|con| con.offset(x, y))
                    .filter(|p| !visited.contains(p)),
            )
        }
        visited.extend(points);
        distance += 1;
    }
    format!("{distance}")
}

pub fn part2(input: &str) -> String {
    let map = parse_input(input);
    let (start_point @ (start_x, start_y), _) = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, con)| ((x, y), con)))
        .find(|(_, con)| con.contains(&Connection::Start))
        .unwrap();

    let (mut current_point, mut prev_dir) = [
        Connection::North,
        Connection::South,
        Connection::East,
        Connection::West,
    ]
    .into_iter()
    .find_map(|dir| {
        let (x, y) = dir.offset(start_x, start_y);
        (map[y][x].contains(&dir.reverse())).then_some(((x, y), dir))
    })
    .unwrap();

    let mut edge_dir = HashMap::new();
    edge_dir.insert(start_point, prev_dir);

    while current_point != start_point {
        let (x, y) = current_point;
        let out_dir = map[y][x]
            .into_iter()
            .find(|&dir| dir != prev_dir.reverse())
            .unwrap();
        let dir = match out_dir {
            Connection::North | Connection::South => out_dir,
            _ => prev_dir,
        };
        edge_dir.insert(current_point, dir);
        prev_dir = out_dir;
        current_point = out_dir.offset(x, y);
    }

    let points = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)));
    let begin_dir = points
        .clone()
        .find_map(|point| {
            let dir = edge_dir.get(&point);
            if let Some(dir @ (Connection::North | Connection::South)) = dir {
                Some(dir)
            } else {
                None
            }
        })
        .unwrap();
    let end_dir = begin_dir.reverse();

    let mut inside = false;
    let mut inside_count = 0;
    for point in points {
        match edge_dir.get(&point) {
            Some(dir) => {
                if dir == begin_dir {
                    inside = true;
                } else if *dir == end_dir {
                    inside = false;
                }
            }
            None => inside_count += inside as u32,
        }
    }
    format!("{inside_count}")
}
