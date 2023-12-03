use core::ops::Range;
use once_cell::sync::Lazy;
use regex::Regex;

fn to_positions(range: Range<usize>, width: usize) -> Vec<(usize, usize)> {
    range.map(|i| (i % width, i / width)).collect()
}

fn adjacent(a: &(usize, usize), b: &(usize, usize)) -> bool {
    a.0.abs_diff(b.0) <= 1 && a.1.abs_diff(b.1) <= 1
}

fn main() {
    let input = include_str!("../day3.txt");

    let width = input.find("\n").unwrap();

    let input = input.replace("\n", "");

    static RE1: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    static RE2: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\.\d]").unwrap());

    let part_numbers: Vec<_> = RE1
        .find_iter(&input)
        .map(|m| {
            (
                to_positions(m.range(), width),
                m.as_str().parse::<u32>().unwrap(),
            )
        })
        .collect();

    let symbols: Vec<_> = RE2
        .find_iter(&input)
        .map(|m| (to_positions(m.range(), width)[0], m.as_str()))
        .collect();

    let gear_symbols: Vec<_> = symbols.iter().filter(|s| s.1 == "*").collect();

    let gear_ratios: Vec<_> = gear_symbols
        .iter()
        .map(|(position, _)| {
            part_numbers
                .iter()
                .filter(|(positions, _)| positions.iter().any(|p| adjacent(p, position)))
                .map(|(_, number)| number)
                .collect::<Vec<_>>()
        })
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .collect();

    println!("{}", gear_ratios.iter().sum::<u32>());
}
