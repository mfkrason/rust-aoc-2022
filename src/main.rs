//use std::Vec;
use std::fs;
//use std::iter::Sum;
//use std::cmp;

fn main() {
    // Read the input
    let input = fs::read_to_string("./inputs/day2.txt").expect("Can't find inputs/day2.txt");

    // Let generator borrow the string
    //let board = day2::generator(&str);

    println!("Part 1: {}", day2::part1(&input));
    println!("Part 2: {}", day2::part2(&input));

}

pub mod day2 {
    use std::collections::HashMap;


    pub fn part1(input: &String) -> u32 {
        let winloss = HashMap::from([
            // Wins
            ("A Y", 2 + 6),
            ("B Z", 3 + 6),
            ("C X", 1 + 6),

             // Draws
            ("A X", 1 + 3),
            ("B Y",2 + 3),
            ("C Z", 3 + 3),
             // Losses
             ("A Z", 3 + 0),
             ("B X", 1 + 0),
             ("C Y", 2 + 0)
        ]);

        let games: Vec<u32> = input
            .lines()
            .map(|x| winloss[&x])
            .collect();

        games.iter().sum()
    }

    pub fn part2(input: &String) -> u32 {
        let winloss = HashMap::from([
            // Lose
            ("A X", 3 + 0),
            ("B X", 1 + 0),
            ("C X", 2 + 0),

            // Draw
            ("A Y", 1 + 3),
            ("B Y", 2 + 3),
            ("C Y", 3 + 3),

            // Win
            ("A Z", 2 + 6),
            ("B Z", 3 + 6),
            ("C Z", 1 + 6),
        ]);

        let games: Vec<u32> = input
            .lines()
            .map(|x| winloss[&x])
            .collect();

        games.iter().sum()
    }

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
