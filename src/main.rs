//use std::Vec;
use std::fs;
//use std::iter::Sum;
//use std::cmp;
//
mod day1;
mod day6;

fn main() {
    // Read the input
    let input = 
        fs::read_to_string(
            "./inputs/day6.txt")
                    .expect("Can't find inputs/day6.txt");

    // Let generator borrow the string
    //let board = day6::generator(&str);

    let generated = crate::day6::day6::generator(&input);

    println!("Part 1: {:#?}", crate::day6::day6::part1(&generated));
    println!("Part 2: {:#?}", crate::day6::day6::part2(&generated));

}


