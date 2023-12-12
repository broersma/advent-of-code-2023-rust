use itertools::Itertools;
use regex::Regex;

fn race(hold_time: u32, race_time: u32) -> u32 {
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

    let input: (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect()
        })
        .collect_tuple()
        .unwrap();

    let (times, distances) = input;

    let product: u32 = times
        .into_iter()
        .zip(distances)
        .map(|(race_time, record)| {
            let count: u32 = (1..race_time)
                .filter(|hold_time| race(*hold_time, race_time) > record)
                .count()
                .try_into()
                .unwrap();
            count
        })
        .product();

    println!("{}", product);
}
