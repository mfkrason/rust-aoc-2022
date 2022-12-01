//use std::Vec;
use std::fs;
//use std::iter::Sum;
//use std::cmp;

fn main() {
    // Read the input
    let str = fs::read_to_string("./inputs/day1.txt").expect("Can't find inputs/day1.txt");

    // Let generator borrow the string
    let elves = day1::generator(&str);

    println!("Part 1: {}", day1::part1(&elves));
    println!("Part 2: {}", day1::part2(&elves));

}

pub mod day1 {
    pub type Int = u32;

    // Parse the file
    pub fn generator(input: &String) -> Vec<Int> {
        // Split on blank lines -- end of line followed by an end of line
        let mut elves: Vec<Int> = input.split("\n\n")
            // For each block
            .map( |x| 
                  // Call lines
                  x.lines()
                  // And for each line in lines parse to an int
                  .map(|x| x.parse::<Int>().expect("not an int"))
                  // Collect it to a Vec<Int>
                  .collect::<Vec<Int>>()
                  // Iterate over that and do the summation
                  .iter()
                  .sum()
                  // And convert that to a vector of ints
                ).collect();

        // Sort in reverse order (normal sort is a.cmp(b)
        elves.sort_by(|a,b| b.cmp(a));
        // "yield" elves
        elves
    }

    // Part 1 is the largest value
    pub fn part1(input: &Vec<Int>) -> Int {
        input[0]
    }

    // Part 2 yields the top 3 values added together
    pub fn part2(input: &Vec<Int>) -> Int {
        input[0] + input[1] + input[2]
    }
}
