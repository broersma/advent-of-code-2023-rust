pub fn main() {
    println!(
        "{:?}",
        include_str!("../day1.txt")
            .lines()
            .map(|l| l.chars()
                      .map(|d| d.to_digit(10))
                      .flatten()
                      .collect::<Vec<u32>>())
            .map(|ns| ns.first().unwrap() * 10 + ns.last().unwrap())
            .sum::<u32>()
    );
}

