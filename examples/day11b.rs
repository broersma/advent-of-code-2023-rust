use itertools::Itertools;
use pathfinding::grid::Grid;
use regex::Regex;
use std::ops::Range;

fn to_position(range: Range<usize>, width: usize) -> (usize, usize) {
    (range.start % width, range.start / width)
}

fn main() {
    let input = include_str!("../day11.txt");

    let width = input.find("\n").unwrap();

    let input = input.replace("\n", "");

    let re = Regex::new(r"#").unwrap();

    let positions: Vec<(usize, usize)> = re
        .find_iter(&input)
        .map(|m| to_position(m.range(), width))
        .collect();

    let x_values: Vec<usize> = positions.iter().map(|p| p.0).collect();
    let y_values: Vec<usize> = positions.iter().map(|p| p.1).collect();

    let empty_x_values: Vec<usize> = (0..*(x_values.iter().max().unwrap()))
        .filter(|x| !x_values.contains(x))
        .collect();
    let empty_y_values: Vec<usize> = (0..*(y_values.iter().max().unwrap()))
        .filter(|y| !y_values.contains(y))
        .collect();

    let positions: Vec<_> = positions
        .iter()
        .map(|(x, y)| {
            (
                x + empty_x_values.iter().filter(|ex| *ex < x).count() * 999999,
                y + empty_y_values.iter().filter(|ey| *ey < y).count() * 999999,
            )
        })
        .collect();

    let max_width = positions.iter().map(|p| p.0).max().unwrap();
    let max_height = positions.iter().map(|p| p.1).max().unwrap();

    let mut grid = Grid::new(max_width as usize, max_height as usize);

    grid.fill();

    let sum_shortest_distances: usize = positions
        .into_iter()
        .combinations(2)
        .map(|ps| grid.distance(ps[0], ps[1]))
        .sum();

    println!("{:?}", sum_shortest_distances);
}
