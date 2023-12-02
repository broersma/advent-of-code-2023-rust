use aho_corasick::{AhoCorasick, MatchKind};

pub fn main() {
    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replacements = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let ac = AhoCorasick::builder()
                         .match_kind(MatchKind::LeftmostFirst)
                         .build(patterns)
                         .unwrap();

    let ac2 = AhoCorasick::builder()
                         .match_kind(MatchKind::LeftmostFirst)
                         .build(patterns.map(|p| p.chars().rev().collect::<String>()))
                         .unwrap();

    let firsts = include_str!("../day1.txt")
            .lines()
            .map(|l| ac.replace_all(l, replacements))
            .map(|l| l.chars()
                      .map(|d| d.to_digit(10))
                      .flatten()
                      .collect::<Vec<u32>>())
            .map(|ns| ns.first().unwrap().to_owned())
            .sum::<u32>();

    let lasts = include_str!("../day1.txt")
            .lines()
            .map(|l| ac2.replace_all(&l.chars().rev().collect::<String>(), replacements))
            .map(|l| l.chars()
                      .map(|d| d.to_digit(10))
                      .flatten()
                      .collect::<Vec<u32>>())
            .map(|ns| ns.first().unwrap().to_owned())
            .sum::<u32>();

    println!(
        "{:?}",
        firsts * 10 + lasts
        );
}

