use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

fn histogram(data: &[u32]) -> HashMap<u32, usize> {
    let mut histogram = HashMap::new();

    for &value in data.iter() {
        let counter = histogram.entry(value).or_insert(0);
        *counter += 1;
    }

    histogram
}

fn total_cards(num_matches: &HashMap<usize, usize>, card_num: &usize) -> usize {
    let num_extra_cards = num_matches[card_num];

    let mut cards = 1;

    for i in 1..=num_extra_cards {
        let extra_cards = total_cards(num_matches, &(card_num + i));
        cards += extra_cards;
    }

    cards
}

fn main() {
    let input = include_str!("../day4.txt");

    static RE1: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    let card_num_matches: HashMap<usize, usize> = input
        .lines()
        .map(|l| l.split(":").skip(1).join(" "))
        .map(|l| {
            RE1.find_iter(&l)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|ns| {
            histogram(&ns)
                .into_iter()
                .filter(|(_, count)| *count > 1)
                .count()
        })
        .enumerate()
        .collect();

    println!(
        "{:#?}",
        card_num_matches
            .keys()
            .map(|card_num| total_cards(&card_num_matches, &card_num))
            .sum::<usize>()
    );
}
