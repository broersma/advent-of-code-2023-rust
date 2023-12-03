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

    let real_part_numbers: Vec<_> = part_numbers
        .iter()
        .filter(|(positions, _)| {
            positions
                .iter()
                .any(|p| symbols.iter().any(|(position, _)| adjacent(p, position)))
        })
        .collect();

    let part_number_sum: u32 = real_part_numbers.iter().map(|(_, number)| number).sum();

    println!("{part_number_sum}");
}
