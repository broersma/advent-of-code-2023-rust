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

fn main() {
    let input = include_str!("../day4.txt");

    static RE1: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    let numbers: usize = input
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
                .collect::<Vec<_>>()
        })
        .map(|ns| if ns.len() > 0 { 1 << (ns.len() - 1) } else { 0 })
        .sum();

    println!("{:#?}", numbers);
}
