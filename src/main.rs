//use std::Vec;
use std::fs;
//use std::iter::Sum;
//use std::cmp;
//
mod day1;
mod day2;

fn main() {
    // Read the input
    let input = fs::read_to_string("./inputs/day2.txt").expect("Can't find inputs/day2.txt");

    // Let generator borrow the string
    //let board = day2::generator(&str);

    println!("Part 1: {}", crate::day2::day2::part1(&input));
    println!("Part 2: {}", crate::day2::day2::part2(&input));

}


