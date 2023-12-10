use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, one_of},
    combinator::recognize,
    combinator::{eof, map_res},
    multi::many0,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_decimal(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 10)
}

fn decimal1(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
}

fn decimal(input: &str) -> IResult<&str, u64> {
    map_res(decimal1, from_decimal)(input)
}

fn numbers_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, vec) = separated_list1(tag(" "), decimal)(input)?;

    Ok((input, vec))
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, vec) = numbers_list(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, vec))
}

fn map<'a>(tag_value: &str, input: &'a str) -> IResult<&'a str, Vec<Vec<u64>>> {
    let (input, _) = tag(tag_value)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, vecs) = separated_list1(line_ending, numbers_list)(input)?;
    let (input, _) = alt((eof, line_ending))(input)?;

    Ok((input, vecs))
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Vec<u64>>,
    soil_to_fertilizer: Vec<Vec<u64>>,
    fertilizer_to_water: Vec<Vec<u64>>,
    water_to_light: Vec<Vec<u64>>,
    light_to_temperature: Vec<Vec<u64>>,
    temperature_to_humidity: Vec<Vec<u64>>,
    humidity_to_location: Vec<Vec<u64>>,
}

fn input_file(input: &str) -> IResult<&str, Input> {
    let (input, seeds) = seeds(input)?;
    let (input, _) = line_ending(input)?;
    let (input, seed_to_soil) = map("seed-to-soil map:", input)?;
    let (input, _) = line_ending(input)?;
    let (input, soil_to_fertilizer) = map("soil-to-fertilizer map:", input)?;
    let (input, _) = line_ending(input)?;
    let (input, fertilizer_to_water) = map("fertilizer-to-water map:", input)?;
    let (input, _) = line_ending(input)?;
    let (input, water_to_light) = map("water-to-light map:", input)?;
    let (input, _) = line_ending(input)?;
    let (input, light_to_temperature) = map("light-to-temperature map:", input)?;
    let (input, _) = line_ending(input)?;
    let (input, temperature_to_humidity) = map("temperature-to-humidity map:", input)?;
    let (input, _) = line_ending(input)?;
    let (input, humidity_to_location) = map("humidity-to-location map:", input)?;

    Ok((
        input,
        Input {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}

fn mapping(input: u64, map: &Vec<Vec<u64>>) -> u64 {
    let matching_range = map
        .iter()
        .find(|range| range[1] <= input && input < range[1] + range[2]);

    if let Some(range) = matching_range {
        return ((input as i64) + (range[0] as i64 - range[1] as i64)) as u64;
    } else {
        return input;
    }
}

fn seed_to_location(seed: u64, maps: &Input) -> u64 {
    let soil = mapping(seed, &maps.seed_to_soil);
    let fertilizer = mapping(soil, &maps.soil_to_fertilizer);
    let water = mapping(fertilizer, &maps.fertilizer_to_water);
    let light = mapping(water, &maps.water_to_light);
    let temperature = mapping(light, &maps.light_to_temperature);
    let humidity = mapping(temperature, &maps.temperature_to_humidity);
    let location = mapping(humidity, &maps.humidity_to_location);
    return location;
}

fn main() {
    let input = include_str!("../day5.txt");

    let input = input_file(input).unwrap().1;

    let minimum_location = input
        .seeds
        .iter()
        .map(|seed| seed_to_location(*seed, &input))
        .min();

    println!("{:?}", minimum_location.unwrap());
}
