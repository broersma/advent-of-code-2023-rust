use itertools::Itertools;
use regex::Regex;

fn race(hold_time: u64, race_time: u64) -> u64 {
    return if hold_time == 0 || hold_time >= race_time {
        0
    } else {
        let speed = hold_time;
        let race_time = race_time - hold_time;
        speed * race_time
    };
}

fn main() {
    let input = include_str!("../day6.txt");

    let re = Regex::new(r"\d+").unwrap();

    let input: (u64, u64) = input
        .lines()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str())
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let (race_time, record) = input;

    let first_winning_hold_time: u64 = (1..race_time)
        .find_or_first(|hold_time| race(*hold_time, race_time) > record)
        .unwrap();

    let last_winning_hold_time: u64 = (1..race_time)
        .rev()
        .find_or_first(|hold_time| race(*hold_time, race_time) > record)
        .unwrap();

    println!("{}", (last_winning_hold_time - first_winning_hold_time) + 1);
}
