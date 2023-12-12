use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        use Card::*;
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => unimplemented!(),
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut counts = HashMap::new();
    let mut counts_vec = Vec::new();
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let hand = hand.chars().map(Card::from_char).collect::<Vec<_>>();
            for &card in &hand {
                *counts.entry(card).or_insert(0u32) += 1;
            }
            counts_vec.clear();
            counts_vec.extend(counts.drain().map(|(_, count)| count));
            counts_vec.sort_by(|count1, count2| count1.cmp(count2).reverse());
            let score = match counts_vec[0] {
                5 => 6,
                4 => 5,
                3 if counts_vec[1] == 2 => 4,
                3 => 3,
                2 if counts_vec[1] == 2 => 2,
                2 => 1,
                _ => 0,
            };
            let bid = bid.parse::<u64>().unwrap();

            (hand, score, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|(hand1, score1, _), (hand2, score2, _)| {
        score1.cmp(score2).then(hand1.cmp(hand2))
    });
    let sum: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i as u64 + 1) * bid)
        .sum();
    format!("{sum}")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card2 {
    Jack, // !
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card2 {
    fn from_char(c: char) -> Self {
        use Card2::*;
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => unimplemented!(),
        }
    }
}

pub fn part2(input: &str) -> String {
    let mut counts = HashMap::new();
    let mut counts_vec = Vec::new();
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let hand = hand.chars().map(Card2::from_char).collect::<Vec<_>>();
            for &card in &hand {
                *counts.entry(card).or_insert(0u32) += 1;
            }
            let jacks = counts.remove(&Card2::Jack).unwrap_or(0);
            counts_vec.clear();
            counts_vec.extend(counts.drain().map(|(_, count)| count));
            counts_vec.sort_by(|count1, count2| count1.cmp(count2).reverse());
            let score = match counts_vec.get(0).unwrap_or(&0) + jacks {
                5 => 6,
                4 => 5,
                3 if counts_vec[1] == 2 => 4,
                3 => 3,
                2 if counts_vec[1] == 2 => 2,
                2 => 1,
                _ => 0,
            };
            let bid = bid.parse::<u64>().unwrap();

            (hand, score, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|(hand1, score1, _), (hand2, score2, _)| {
        score1.cmp(score2).then(hand1.cmp(hand2))
    });
    let sum: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i as u64 + 1) * bid)
        .sum();
    format!("{sum}")
}
