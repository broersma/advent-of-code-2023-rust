use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct Reveal {
    r: u32,
    g: u32,
    b: u32,
}

fn parse_reveal(input: &str) -> Reveal {
    static re: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (\w+)").unwrap());
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    re.captures_iter(input)
        .map(|c| c.extract())
        .for_each(|(_, [num, color])| match color {
            "red" => r = num.parse::<u32>().unwrap(),
            "green" => g = num.parse::<u32>().unwrap(),
            "blue" => b = num.parse::<u32>().unwrap(),
            &_ => panic!(),
        });

    Reveal { r, g, b }
}

fn main() {
    let limit = Reveal {
        r: 12,
        g: 13,
        b: 14,
    };
    println!(
        "{:?}",
        include_str!("../day2.txt")
            .lines()
            .map(|l| l
                .split(":")
                .last()
                .unwrap()
                .split(";")
                .map(|r| parse_reveal(r))
                .fold(Reveal { r: 0, g: 0, b: 0 }, |acc, r| Reveal {
                    r: max(acc.r, r.r),
                    g: max(acc.g, r.g),
                    b: max(acc.b, r.b)
                }))
            .map(|r| r.r * r.g * r.b)
            .sum::<u32>()
    );
}
